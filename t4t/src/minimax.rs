use crate::{Finite, Game, GameTree, Move, Outcome, Playable, PlayerIndex, State, Utility};
use std::sync::Arc;

// TODO
#[allow(missing_docs)]

/// Builds a strategy that uses the [expectiminimax](https://en.wikipedia.org/wiki/Expectiminimax)
/// algorithm to choose the move that maximizes the minimum utility of the possible outcomes for the
/// player.
///
/// The algorithm uses [alpha-beta pruning](https://en.wikipedia.org/wiki/Alpha%E2%80%93beta_pruning)
/// to reduce the search space where possible.
///
/// # TODO
/// The "expecti-" part of the algorithm is not yet implemented. That is, the strategy will panic if
/// it encounters a chance node in the game tree.
pub struct Minimax<G: Game<P> + Finite<P>, const P: usize> {
    game: Arc<G>,
    player: PlayerIndex<P>,
    max_depth: usize,
    #[allow(clippy::type_complexity)]
    heuristic: Arc<dyn Fn(&G::State) -> f64>,
}

#[allow(dead_code)]
impl<M, S, U, O, G, const P: usize> Minimax<G, P>
where
    M: Move,
    S: State,
    U: Utility + Into<f64>,
    O: Outcome<M, U, P>,
    G: Game<P, Move = M, Utility = U, State = S, View = S> + Finite<P> + Playable<P, Outcome = O>,
{
    /// Create a new minimax strategy for the given game and player.
    ///
    /// The heuristic function will be applied to the game state when the maximum search depth is
    /// reached. The heuristic function should return a value that is between the minimum and
    /// maximum payoff values achievable by the player.
    pub fn new(
        game: &Arc<G>,
        player: PlayerIndex<P>,
        max_depth: usize,
        heuristic: impl Fn(&S) -> f64 + 'static,
    ) -> Self {
        Minimax {
            game: Arc::clone(game),
            player,
            max_depth,
            heuristic: Arc::new(heuristic),
        }
    }

    /// Create a new minimax strategy for the given game and player with no maximum search depth.
    /// The strategy will always perform a total search of the game tree.
    pub fn total(game: &Arc<G>, player: PlayerIndex<P>) -> Self {
        Self::new(game, player, usize::MAX, |_| 0.0)
    }

    fn value(
        &self,
        my_index: PlayerIndex<P>,
        node: &GameTree<S, M, U, O, P>,
        mut alpha: f64,
        mut beta: f64,
        depth: usize,
    ) -> f64 {
        match node.clone().sequentialize() {
            GameTree::Turns {
                state,
                to_move,
                next,
            } => {
                assert_eq!(to_move.len(), 1);
                let player = to_move[0];
                if depth >= self.max_depth {
                    (self.heuristic)(&state)
                } else if alpha >= beta {
                    if player == my_index {
                        alpha
                    } else {
                        beta
                    }
                } else if player == my_index {
                    // maximizing player
                    let mut value = f64::NEG_INFINITY;
                    for the_move in self.game.possible_moves(player, &state) {
                        let child = next(state.clone(), vec![the_move])
                            .expect("malformed game tree: possible move is invalid");
                        let child_value = self.value(my_index, &child, alpha, beta, depth + 1);
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
                    for the_move in self.game.possible_moves(player, &state) {
                        let child = next(state.clone(), vec![the_move])
                            .expect("malformed game tree: possible move is invalid");
                        let child_value = self.value(my_index, &child, alpha, beta, depth + 1);
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
                if depth >= self.max_depth {
                    (self.heuristic)(&state)
                } else {
                    todo! {"Minimax with chance nodes not yet implemented"}
                }
            }
            GameTree::End { outcome, .. } => outcome.payoff()[self.player].into(),
        }
    }
}
