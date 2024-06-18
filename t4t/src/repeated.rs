use std::fmt;
use std::sync::Arc;

use crate::{Action, Game, GameTree, History, PlayerIndex};

/// A finitely [repeated](https://en.wikipedia.org/wiki/Repeated_game) or iterated version of game
/// `G`.
///
/// Game `G` is called the "stage game". This game plays the stage game a specified number of times,
/// accumulating the payoffs.
#[derive(Clone)]
pub struct Repeated<G: Game<P>, const P: usize> {
    stage_game: Arc<G>,
    repetitions: usize,
}

/// The intermediate state of a repeated game.
#[derive(Clone)]
pub struct RepeatedState<G: Game<P>, const P: usize> {
    stage_game: Arc<G>,
    stage_state: Arc<G::State>,
    completed: History<G, P>,
    remaining: usize,
}

impl<G: Game<P> + 'static, const P: usize> Repeated<G, P> {
    /// Construct a repeated game that plays the stage game the given number of repetitions.
    pub fn new(stage_game: Arc<G>, repetitions: usize) -> Self {
        Repeated {
            stage_game,
            repetitions,
        }
    }

    /// Get the stage game for this repeated game.
    pub fn stage_game(&self) -> &Arc<G> {
        &self.stage_game
    }

    /// Get the number of repetitions the stage game will be played.
    pub fn repetitions(&self) -> usize {
        self.repetitions
    }
}

impl<G: Game<P>, const P: usize> RepeatedState<G, P> {
    /// Construct a new repeated game state.
    pub fn new(stage_game: Arc<G>, remaining: usize) -> Self {
        let stage_state = stage_game.game_tree().state.clone();
        RepeatedState {
            stage_game,
            stage_state,
            completed: History::empty(),
            remaining,
        }
    }

    /// Get the view of the stage game's current intermediate state for the given player.
    pub fn state_view(&self, player: PlayerIndex<P>) -> G::View {
        self.stage_game
            .state_view(self.stage_state.as_ref(), player)
    }

    /// The current history of all completed repetitions of the stage game so far.
    pub fn history(&self) -> &History<G, P> {
        &self.completed
    }

    /// The number of remaining repetitions of the stage game to play.
    pub fn remaining(&self) -> usize {
        self.remaining
    }
}

fn lift_node<'g, G: Game<P> + 'g, const P: usize>(
    stage_game: &'g G,
    state: Arc<RepeatedState<G, P>>,
    node: GameTree<'g, G::State, G::Move, G::Outcome, P>,
) -> GameTree<'g, RepeatedState<G, P>, G::Move, History<G, P>, P> {
    match node.action {
        Action::Turns {
            to_move: players,
            next,
        } => GameTree::players(
            state.clone(),
            players,
            move |repeated_state: Arc<RepeatedState<G, P>>, moves: Vec<G::Move>| match next(
                repeated_state.stage_state.clone(),
                moves,
            ) {
                Ok(stage_node) => {
                    let mut next_state = (*state).clone();
                    next_state.stage_state = stage_node.state.clone();

                    Ok(lift_node(stage_game, Arc::new(next_state), stage_node))
                }

                Err(kind) => Err(kind),
            },
        ),

        Action::Chance { distribution, next } => GameTree::chance(
            state.clone(),
            distribution,
            move |repeated_state: Arc<RepeatedState<G, P>>, the_move: G::Move| match next(
                repeated_state.stage_state.clone(),
                the_move,
            ) {
                Ok(stage_node) => {
                    let mut next_state = (*state).clone();
                    next_state.stage_state = stage_node.state.clone();

                    Ok(lift_node(stage_game, Arc::new(next_state), stage_node))
                }

                Err(kind) => Err(kind),
            },
        ),

        Action::End { outcome } if state.remaining > 0 => {
            let stage_node = stage_game.game_tree();

            let mut next_state = (*state).clone();
            next_state.stage_state = stage_node.state.clone();

            next_state.completed.add(outcome);
            next_state.remaining -= 1;

            lift_node(stage_game, Arc::new(next_state), stage_node)
        }

        Action::End { outcome } => {
            let mut history = state.completed.clone(); // TODO avoid this clone
            history.add(outcome);

            GameTree::end(state, history)
        }
    }
}

impl<G: Game<P> + 'static, const P: usize> Game<P> for Repeated<G, P> {
    type Move = G::Move;
    type Utility = G::Utility;
    type Outcome = History<G, P>;
    type State = RepeatedState<G, P>;
    type View = RepeatedState<G, P>; // TODO add RepeatedStateView or some other solution

    fn game_tree(&self) -> GameTree<RepeatedState<G, P>, G::Move, History<G, P>, P> {
        let init_state = Arc::new(RepeatedState::new(
            self.stage_game.clone(),
            self.repetitions - 1,
        ));

        lift_node(
            self.stage_game.as_ref(),
            init_state,
            self.stage_game.game_tree(),
        )
    }

    fn state_view(
        &self,
        state: &RepeatedState<G, P>,
        _player: PlayerIndex<P>,
    ) -> RepeatedState<G, P> {
        state.clone() // TODO
    }

    fn is_valid_move(
        &self,
        state: &Self::State,
        player: PlayerIndex<P>,
        the_move: Self::Move,
    ) -> bool {
        self.stage_game
            .is_valid_move(&state.stage_state, player, the_move)
    }
}

impl<G: Game<P>, const P: usize> fmt::Debug for RepeatedState<G, P> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("RepeatedState")
            .field("stage_state", &self.stage_state)
            .field("completed", &self.completed)
            .field("remaining", &self.remaining)
            .finish()
    }
}

impl<G: Game<P>, const P: usize> PartialEq for RepeatedState<G, P> {
    fn eq(&self, other: &Self) -> bool {
        self.stage_state == other.stage_state
            && self.completed == other.completed
            && self.remaining == other.remaining
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Normal;
    use impls::impls;
    use test_log::test;

    #[test]
    fn repeated_is_send_sync() {
        assert!(impls!(Repeated<Normal<(), u8, 2>, 2>: Send & Sync));
    }
}
