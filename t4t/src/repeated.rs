use std::fmt;
use std::sync::Arc;

use crate::{Action, Finite, Game, GameTree, History, Playable, PlayerIndex, PossibleMoves};

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
pub struct RepeatedState<G: Playable<P>, const P: usize> {
    stage_game: Arc<G>,
    stage_state: G::State,
    completed: Arc<History<G, P>>,
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

impl<G: Playable<P>, const P: usize> RepeatedState<G, P> {
    /// Construct a new repeated game state.
    pub fn new(stage_game: Arc<G>, remaining: usize) -> Self {
        let stage_state = stage_game.game_tree().state;
        RepeatedState {
            stage_game,
            stage_state,
            completed: Arc::new(History::empty()),
            remaining,
        }
    }

    /// Get the view of the stage game's current intermediate state for the given player.
    pub fn state_view(&self, player: PlayerIndex<P>) -> G::View {
        self.stage_game.state_view(&self.stage_state, player)
    }

    /// The current history of all completed repetitions of the stage game so far.
    pub fn history(&self) -> &Arc<History<G, P>> {
        &self.completed
    }

    /// The number of remaining repetitions of the stage game to play.
    pub fn remaining(&self) -> usize {
        self.remaining
    }
}

fn generate_tree<G: Playable<P> + 'static, const P: usize>(
    stage_game: Arc<G>,
    state: RepeatedState<G, P>,
    action: Action<G::State, G::Move, G::Utility, G::Outcome, P>,
) -> GameTree<RepeatedState<G, P>, G::Move, G::Utility, Arc<History<G, P>>, P> {
    let remaining = state.remaining;
    match action {
        Action::Turns {
            to_move: players,
            next,
        } => GameTree::players(
            state,
            players,
            move |current_state: RepeatedState<G, P>, moves: Vec<G::Move>| match next(
                current_state.stage_state,
                moves,
            ) {
                Ok(stage_tree) => {
                    let next_state = RepeatedState {
                        stage_game: Arc::clone(&stage_game),
                        stage_state: stage_tree.state,
                        completed: Arc::clone(&current_state.completed),
                        remaining,
                    };

                    Ok(generate_tree(
                        Arc::clone(&stage_game),
                        next_state,
                        stage_tree.action,
                    ))
                }

                Err(kind) => Err(kind),
            },
        ),

        Action::Chance { distribution, next } => GameTree::chance(
            state,
            distribution,
            move |current_state: RepeatedState<G, P>, the_move: G::Move| match next(
                current_state.stage_state,
                the_move,
            ) {
                Ok(stage_tree) => {
                    let next_state = RepeatedState {
                        stage_game: Arc::clone(&stage_game),
                        stage_state: stage_tree.state,
                        completed: Arc::clone(&current_state.completed),
                        remaining,
                    };

                    Ok(generate_tree(
                        Arc::clone(&stage_game),
                        next_state,
                        stage_tree.action,
                    ))
                }

                Err(kind) => Err(kind),
            },
        ),

        Action::End { outcome, .. } if remaining > 0 => {
            let stage_tree = stage_game.game_tree();

            let mut completed = Arc::unwrap_or_clone(state.completed);
            completed.add(outcome);

            let next_state = RepeatedState {
                stage_game: Arc::clone(&stage_game),
                stage_state: stage_tree.state,
                completed: Arc::new(completed),
                remaining: remaining - 1,
            };

            generate_tree(stage_game, next_state, stage_tree.action)
        }

        Action::End { outcome, .. } => {
            let mut completed = Arc::unwrap_or_clone(state.completed);
            completed.add(outcome);

            let arc_completed = Arc::new(completed);

            let final_state = RepeatedState {
                stage_game: Arc::clone(&stage_game),
                stage_state: state.stage_state,
                completed: Arc::clone(&arc_completed),
                remaining,
            };

            GameTree::end(final_state, arc_completed)
        }
    }
}

impl<G: Playable<P> + 'static, const P: usize> Game<P> for Repeated<G, P> {
    type Move = G::Move;
    type Utility = G::Utility;
    type State = RepeatedState<G, P>;
    type View = RepeatedState<G, P>; // TODO add RepeatedStateView or some other solution

    fn state_view(
        &self,
        state: &RepeatedState<G, P>,
        _player: PlayerIndex<P>,
    ) -> RepeatedState<G, P> {
        state.clone() // TODO
    }
}

impl<G: Playable<P> + 'static, const P: usize> Playable<P> for Repeated<G, P> {
    type Outcome = Arc<History<G, P>>;

    fn into_game_tree(
        self,
    ) -> GameTree<RepeatedState<G, P>, G::Move, G::Utility, Arc<History<G, P>>, P> {
        let init_state = RepeatedState::new(Arc::clone(&self.stage_game), self.repetitions - 1);
        let init_action = self.stage_game.game_tree().action;

        generate_tree(Arc::clone(&self.stage_game), init_state, init_action)
    }
}

impl<G: Playable<P> + Finite<P> + 'static, const P: usize> Finite<P> for Repeated<G, P> {
    fn possible_moves(
        &self,
        player: PlayerIndex<P>,
        state: &Self::State,
    ) -> PossibleMoves<'_, Self::Move> {
        self.stage_game.possible_moves(player, &state.stage_state)
    }
}

impl<G: Playable<P>, const P: usize> fmt::Debug for RepeatedState<G, P> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("RepeatedState")
            .field("stage_state", &self.stage_state)
            .field("completed", &self.completed)
            .field("remaining", &self.remaining)
            .finish()
    }
}

impl<G: Playable<P>, const P: usize> PartialEq for RepeatedState<G, P> {
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
