use std::fmt;
use std::sync::Arc;

use crate::{Finite, Game, GameTree, History, InvalidMove, Playable, PlayerIndex, PossibleMoves};

/// Game tree type for a [`Repeated`] game with stage game `G`.
pub type RepeatedGameTree<G, const P: usize> = GameTree<
    RepeatedState<G, P>,
    <G as Game<P>>::Move,
    <G as Game<P>>::Utility,
    History<<G as Game<P>>::Move, <G as Game<P>>::Utility, <G as Playable<P>>::Outcome, P>,
    P,
>;

/// A finitely [repeated](https://en.wikipedia.org/wiki/Repeated_game) or iterated version of game
/// `G`.
///
/// Game `G` is called the "stage game". This game plays the stage game a specified number of times,
/// accumulating the payoffs.
#[derive(Clone)]
pub struct Repeated<G: Game<P>, const P: usize> {
    stage_game: G,
    repetitions: usize,
}

/// The intermediate state of a repeated game.
#[derive(Clone)]
pub struct RepeatedState<G: Playable<P>, const P: usize> {
    stage_state: G::State,
    completed: History<G::Move, G::Utility, G::Outcome, P>,
    remaining: usize,
}

impl<G: Game<P> + 'static, const P: usize> Repeated<G, P> {
    /// Construct a repeated game that plays the stage game the given number of repetitions.
    pub fn new(stage_game: G, repetitions: usize) -> Self {
        Repeated {
            stage_game,
            repetitions,
        }
    }

    /// Get the stage game for this repeated game.
    pub fn stage_game(&self) -> &G {
        &self.stage_game
    }

    /// Get the number of repetitions the stage game will be played.
    pub fn repetitions(&self) -> usize {
        self.repetitions
    }
}

impl<G: Playable<P>, const P: usize> RepeatedState<G, P> {
    /// Construct a new repeated game state.
    pub fn new(stage_game: &G, remaining: usize) -> Self {
        let stage_state = stage_game.game_tree().state().clone();
        RepeatedState {
            stage_state,
            completed: History::empty(),
            remaining,
        }
    }

    /// The current history of all completed repetitions of the stage game so far.
    pub fn history(&self) -> &History<G::Move, G::Utility, G::Outcome, P> {
        &self.completed
    }

    /// The number of remaining repetitions of the stage game to play.
    pub fn remaining(&self) -> usize {
        self.remaining
    }
}

fn generate_tree<G: Playable<P> + 'static, const P: usize>(
    stage_game: Arc<G>,
    stage_node: GameTree<G::State, G::Move, G::Utility, G::Outcome, P>,
    repeated_state: RepeatedState<G, P>,
) -> RepeatedGameTree<G, P> {
    let remaining = repeated_state.remaining;
    match stage_node {
        GameTree::Turns {
            state: stage_state,
            to_move,
            next,
        } => GameTree::players(
            repeated_state,
            to_move,
            move |current_state: RepeatedState<G, P>, moves: Vec<G::Move>| {
                // TODO: not sure why I need to clone stage_state here...
                let next_stage_tree = next(stage_state.clone(), moves).map_err(|err| {
                    InvalidMove::new(current_state.clone(), err.player, err.the_move)
                })?;

                let next_stage_state = next_stage_tree.state().clone();
                let next_repeated_state = RepeatedState {
                    stage_state: next_stage_state,
                    completed: current_state.completed.clone(),
                    remaining,
                };

                Ok(generate_tree(
                    Arc::clone(&stage_game),
                    next_stage_tree,
                    next_repeated_state,
                ))
            },
        ),

        GameTree::Chance {
            state: stage_state,
            distribution,
            next,
        } => GameTree::chance(
            repeated_state,
            distribution,
            move |current_state: RepeatedState<G, P>, the_move: G::Move| {
                // TODO: not sure why I need to clone stage_state here...
                let next_stage_tree = next(stage_state.clone(), the_move).map_err(|err| {
                    InvalidMove::new(current_state.clone(), err.player, err.the_move)
                })?;

                let next_stage_state = next_stage_tree.state().clone();
                let next_repeated_state = RepeatedState {
                    stage_state: next_stage_state,
                    completed: current_state.completed.clone(),
                    remaining,
                };

                Ok(generate_tree(
                    Arc::clone(&stage_game),
                    next_stage_tree,
                    next_repeated_state,
                ))
            },
        ),

        GameTree::End { outcome, .. } if remaining > 0 => {
            let mut completed = repeated_state.completed.clone();
            completed.add(outcome);

            let next_stage_tree = stage_game.game_tree();
            let next_stage_state = next_stage_tree.state().clone();
            let next_repeated_state = RepeatedState {
                stage_state: next_stage_state,
                completed,
                remaining: remaining - 1,
            };

            generate_tree(stage_game, next_stage_tree, next_repeated_state)
        }

        GameTree::End {
            state: stage_state,
            outcome,
            ..
        } => {
            let mut completed = repeated_state.completed.clone();
            completed.add(outcome);

            let final_state = RepeatedState {
                stage_state,
                completed: completed.clone(),
                remaining,
            };

            GameTree::end(final_state, completed)
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
    type Outcome = History<G::Move, G::Utility, G::Outcome, P>;

    fn into_game_tree(self) -> RepeatedGameTree<G, P> {
        let init_state = RepeatedState::new(&self.stage_game, self.repetitions - 1);
        let stage_tree = self.stage_game.game_tree();

        generate_tree(Arc::new(self.stage_game), stage_tree, init_state)
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
