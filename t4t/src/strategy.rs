use crate::{Distribution, Finite, GameTree, Move, Outcome, Playable, PlayerIndex, State, Utility};
use ordered_float::OrderedFloat;
use std::sync::Arc;

/// The strategic context in which a player makes a move during a game.
///
/// This type includes all information, besides the definition of the stage game, that a strategy
/// may use to compute its next move. It includes the player's index and the player's view of the
/// game state.
#[derive(Clone)]
pub struct Context<'a, G: Playable<P>, const P: usize> {
    game: &'a G,
    location: &'a GameTree<G::State, G::Move, G::Utility, G::Outcome, P>,
    state_view: &'a G::View,
    index: PlayerIndex<P>,
}

/// A function computing the next move for a player given a strategic context.
///
/// This trait is effectively a type synonym for the function type it extends. A blanket
/// implementation covers all possible instances, so it should not be implemented directly.
pub trait NextMove<G: Playable<P>, const P: usize>:
    FnMut(Context<'_, G, P>) -> G::Move + Send + Sync + 'static
{
}

impl<F, G: Playable<P>, const P: usize> NextMove<G, P> for F where
    F: FnMut(Context<'_, G, P>) -> G::Move + Send + Sync + 'static
{
}

impl<'a, G: Playable<P>, const P: usize> Context<'a, G, P> {
    /// Construct a new context from the index of the player whose turn it is to move and that
    /// player's view of the current state.
    pub fn new(
        game: &'a G,
        location: &'a GameTree<G::State, G::Move, G::Utility, G::Outcome, P>,
        state_view: &'a G::View,
        index: PlayerIndex<P>,
    ) -> Self {
        Context {
            game,
            location,
            state_view,
            index,
        }
    }

    /// The game being played.
    pub fn game(&self) -> &'a G {
        self.game
    }

    /// Get the player's view of the current state of the game.
    pub fn state_view(&self) -> &'a G::View {
        self.state_view
    }

    /// Get the index of the player whose turn it is to move. The method is named "my" from the
    /// perspective of the strategy that receives this context.
    pub fn my_index(&self) -> PlayerIndex<P> {
        self.index
    }
}

impl<'a, G: Playable<2>> Context<'a, G, 2> {
    /// Get the index of the other player in a two player game. The method is named "their"
    /// (singular) from the perspective of the strategy that receives this context.
    pub fn their_index(&self) -> PlayerIndex<2> {
        self.index.next()
    }
}

impl<'a, S, G: Playable<P, State = S, View = S>, const P: usize> Context<'a, G, P> {
    /// Get the current location in the game tree.
    ///
    /// # Note
    /// This method should only be used in strategies for
    /// [perfect-information](https://en.wikipedia.org/wiki/Perfect_information) games, that is,
    /// games where the player's view of the state is the same as the state itself.
    ///
    /// Game implementors can ensure that this method is unavailable for games with imperfect
    /// information by making the state and view types different.
    pub fn location(&self) -> &'a GameTree<S, G::Move, G::Utility, G::Outcome, P> {
        self.location
    }
}

/// A strategy is a function from an intermediate game context to a move.
pub struct Strategy<G: Playable<P>, const P: usize> {
    #[allow(clippy::type_complexity)]
    next_move: Box<dyn NextMove<G, P>>,
}

impl<G: Playable<P> + 'static, const P: usize> Strategy<G, P> {
    /// Construct a new strategy from a function that computes the next move given a strategic
    /// context.
    pub fn new(next_move: impl NextMove<G, P>) -> Self {
        Strategy {
            next_move: Box::new(next_move),
        }
    }

    /// Construct a [pure strategy](https://en.wikipedia.org/wiki/Strategy_(game_theory)#Pure_and_mixed_strategies)
    /// that always plays the same move regardless of the context.
    pub fn pure(the_move: G::Move) -> Self {
        Strategy::new(move |_| the_move)
    }

    /// Construct a [mixed strategy](https://en.wikipedia.org/wiki/Strategy_(game_theory)#Mixed_strategy)
    /// that plays a move according to the given probability distribution over moves.
    pub fn mixed(dist: Distribution<G::Move>) -> Self {
        Strategy::new(move |_| dist.sample().to_owned())
    }

    /// Construct a [mixed strategy](https://en.wikipedia.org/wiki/Strategy_(game_theory)#Mixed_strategy)
    /// from a flat distribution over the given moves. This strategy will pick one move randomly,
    /// each with equal probability.
    ///
    /// # Errors
    ///
    /// Logs an error and returns `None` if:
    /// - The vector is empty.
    /// - The vector is longer than u32::MAX.
    pub fn mixed_flat<I: IntoIterator<Item = G::Move>>(moves: I) -> Option<Self> {
        Distribution::flat(moves).map(|dist| Strategy::mixed(dist))
    }

    /// Construct a probabilistic strategy that plays another strategy according to the given
    /// probability distribution.
    ///
    /// A distribution of pure strategies is equivalent to a [mixed](Strategy::mixed) strategy.
    pub fn probabilistic(mut dist: Distribution<Strategy<G, P>>) -> Self {
        Strategy::new(move |context| dist.sample_mut().next_move(context))
    }

    /// Construct a periodic strategy that plays the given sequence of strategies in order, then
    /// repeats.
    pub fn periodic(mut strategies: Vec<Strategy<G, P>>) -> Self {
        let mut next_index = 0;
        Strategy::new(move |context| {
            let the_move = strategies[next_index].next_move(context);
            next_index = (next_index + 1) % strategies.len();
            the_move
        })
    }
    /// Construct a periodic strategy of pure strategies. That is, play the given moves in order
    /// and repeat indefinitely.
    pub fn periodic_pure(moves: Vec<G::Move>) -> Self {
        let strategies = Vec::from_iter(moves.into_iter().map(|m| Strategy::pure(m)));
        Strategy::periodic(strategies)
    }

    /// Construct a new conditional strategy that plays the `on_true` strategy if `condition`
    /// returns true for the current context, and plays the `on_false` strategy otherwise.
    pub fn conditional(
        mut condition: impl FnMut(Context<G, P>) -> bool + Send + Sync + 'static,
        mut on_true: Strategy<G, P>,
        mut on_false: Strategy<G, P>,
    ) -> Self {
        Strategy::new(move |context| {
            if condition(context.clone()) {
                on_true.next_move(context)
            } else {
                on_false.next_move(context)
            }
        })
    }

    /// Construct a new trigger strategy that plays the `before` strategy until `trigger` returns
    /// true, then plays the `after` strategy thereafter.
    pub fn trigger(
        mut trigger: impl FnMut(Context<G, P>) -> bool + Send + Sync + 'static,
        mut before: Strategy<G, P>,
        mut after: Strategy<G, P>,
    ) -> Self {
        let mut triggered = false;
        Strategy::new(move |context| {
            if !triggered {
                triggered = trigger(context.clone());
            }
            if triggered {
                after.next_move(context)
            } else {
                before.next_move(context)
            }
        })
    }

    /// Get the next move to play given the current play context.
    pub fn next_move(&mut self, context: Context<G, P>) -> G::Move {
        (self.next_move)(context)
    }
}

impl<S, G, const P: usize> Strategy<G, P>
where
    S: State,
    G: Finite<P, State = S, View = S> + Playable<P>,
{
    /// For a finite [perfect-information](https://en.wikipedia.org/wiki/Perfect_information) game,
    /// construct a strategy that chooses a move randomly from the set of possible moves.
    ///
    /// # Panics
    ///
    /// Panics if the number of possible moves is 0 or larger than `u32::MAX`.
    pub fn randomly() -> Self {
        Strategy::new(|context: Context<G, P>| {
            let player = context.my_index();
            let state = context.state_view();
            let moves = context.game().possible_moves(player, state);
            let dist = Distribution::flat(moves);
            match dist {
                Some(dist) => dist.sample().to_owned(),
                None => panic!("randomly: Could not build distribution."),
            }
        })
    }
}

impl<S, M, U, O, G, const P: usize> Strategy<G, P>
where
    S: State,
    M: Move,
    U: Utility + Into<f64>,
    O: Outcome<M, U, P>,
    G: Finite<P, Move = M, Utility = U, State = S, View = S> + Playable<P, Outcome = O>,
{
    /// Construct a strategy that uses the
    /// [expectiminimax](https://en.wikipedia.org/wiki/Expectiminimax) algorithm to choose the move
    /// that maximizes the minimum utility of the possible outcomes for the player.
    ///
    /// The algorithm uses [alpha-beta pruning](https://en.wikipedia.org/wiki/Alpha%E2%80%93beta_pruning)
    /// to reduce the search space where possible.
    ///
    /// The heuristic function will be applied to the game state when the maximum search depth is
    /// reached. The heuristic function should return a value that is between the minimum and
    /// maximum payoff values achievable by the player.
    ///
    /// # TODO
    /// The "expecti-" part of the algorithm is not yet implemented. That is, the strategy will
    /// panic if it encounters a chance node in the game tree.
    pub fn minimax(
        max_depth: usize,
        heuristic: impl Fn(&S) -> f64 + Send + Sync + 'static,
    ) -> Self {
        let heuristic = Arc::new(heuristic);
        Strategy::new(move |context: Context<G, P>| {
            let game_tree = context
                .location()
                .clone()
                .sequentialize(Some(context.my_index()));

            let next = if let GameTree::Turn { next, .. } = game_tree {
                next
            } else {
                panic!("minimax: expected a turn node")
            };

            let best_move = context
                .game()
                .possible_moves(context.my_index(), context.state_view())
                .max_by_key(|the_move| {
                    let child = next(context.state_view().clone(), vec![*the_move])
                        .expect("malformed game tree: possible move is invalid");
                    let value = Strategy::minimax_value(
                        context.game(),
                        context.my_index(),
                        Arc::clone(&heuristic),
                        max_depth,
                        &child,
                        f64::NEG_INFINITY,
                        f64::INFINITY,
                    );
                    OrderedFloat(value)
                });

            best_move.expect("minimax: no possible moves")
        })
    }

    /// Construct a version of the [minimax](Strategy::minimax) strategy with no maximum search
    /// depth.
    ///
    /// This strategy will always perform a total search of the game tree starting from the
    /// player's location, and so is only suitable for relatively small games.
    pub fn total_minimax() -> Self {
        Strategy::minimax(usize::MAX, |_| 0.0)
    }

    /// Recursive helper function for the minimax strategy.
    fn minimax_value(
        game: &G,
        my_index: PlayerIndex<P>,
        heuristic: Arc<impl Fn(&S) -> f64>,
        depth: usize,
        node: &GameTree<S, M, U, O, P>,
        mut alpha: f64,
        mut beta: f64,
    ) -> f64 {
        match node.clone().sequentialize(Some(my_index)) {
            GameTree::Turn {
                state,
                to_move,
                next,
            } => {
                assert_eq!(to_move.len(), 1);
                let player = to_move[0];
                if depth == 0 {
                    heuristic(&state)
                } else if alpha >= beta {
                    if player == my_index {
                        alpha
                    } else {
                        beta
                    }
                } else if player == my_index {
                    // maximizing player
                    let mut value = f64::NEG_INFINITY;
                    for the_move in game.possible_moves(player, &state) {
                        let child = next(state.clone(), vec![the_move])
                            .expect("malformed game tree: possible move is invalid");
                        let child_value = Strategy::minimax_value(
                            game,
                            my_index,
                            Arc::clone(&heuristic),
                            depth - 1,
                            &child,
                            alpha,
                            beta,
                        );
                        value = f64::max(value, child_value);
                        alpha = f64::max(alpha, value);
                        if value >= beta {
                            break;
                        }
                    }
                    value
                } else {
                    // minimizing player
                    let mut value = f64::INFINITY;
                    for the_move in game.possible_moves(player, &state) {
                        let child = next(state.clone(), vec![the_move])
                            .expect("malformed game tree: possible move is invalid");
                        let child_value = Strategy::minimax_value(
                            game,
                            my_index,
                            Arc::clone(&heuristic),
                            depth - 1,
                            &child,
                            alpha,
                            beta,
                        );
                        value = f64::min(value, child_value);
                        beta = f64::min(beta, value);
                        if value <= alpha {
                            break;
                        }
                    }
                    value
                }
            }

            GameTree::Chance {
                state,
                distribution: _,
                next: _,
            } => {
                if depth == 0 {
                    heuristic(&state)
                } else {
                    todo! {"Minimax with chance nodes not yet implemented"}
                }
            }

            GameTree::End { outcome, .. } => outcome.payoff()[my_index].into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Normal;
    use impls::impls;
    use test_log::test;

    #[test]
    fn strategy_is_send_sync() {
        assert!(impls!(Strategy<Normal<(), u8, 2>, 2>: Send & Sync));
    }
}
