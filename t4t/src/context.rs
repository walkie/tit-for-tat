use crate::{Game, History, Move, Outcome, Payoff, PlayerIndex, State, Transcript, Utility};
use std::fmt::Debug;
use std::mem;

/// The strategic context in which a player makes a move during a repeated game.
///
/// This type includes all information, besides the definition of the stage game, that a strategy
/// for a repeated game may use to compute its next move. It includes the history of past games
/// played, the game state of the current iteration, and (for sequential games) the transcript of
/// moves played so far in the current iteration.
pub struct Context<S, M, U, const P: usize> {
    current_player: Option<PlayerIndex<P>>,
    game_state: Option<S>,
    transcript: Transcript<M, P>,
    score: Payoff<U, P>,
}

impl<S: State, M: Move, U: Utility, const P: usize> Context<S, M, U, P> {
    pub(crate) fn new(initial_state: S) -> Self {
        Context {
            current_player: None,
            game_state: Some(initial_state),
            transcript: Transcript::new(),
            score: Payoff::zeros(),
        }
    }

    pub(crate) fn set_current_player(&mut self, player: PlayerIndex<P>) {
        self.current_player = Some(player);
    }

    pub(crate) fn unset_current_player(&mut self) {
        self.current_player = None;
    }

    pub(crate) fn set_game_state(&mut self, state: S) {
        self.game_state = Some(state);
    }

    pub(crate) fn take_game_state(&mut self) -> Option<S> {
        mem::replace(&mut self.game_state, None)
    }

    pub(crate) fn update_game_state(&mut self, update: impl FnOnce(S) -> Option<S>) {
        if let Some(state) = Option::take(&mut self.game_state) {
            self.game_state = update(state);
        }
    }

    pub fn complete(
        &mut self,
        outcome: Outcome<G::Kind, G::Move, G::Utility, P>,
    ) -> &Outcome<G::Kind, G::Move, G::Utility, P> {
        self.history.add(outcome)
    }

    pub fn current_player(&self) -> Option<PlayerIndex<P>> {
        self.current_player
    }

    pub fn game_state(&self) -> Option<&G::State> {
        self.game_state.as_ref()
    }

    pub fn in_progress(&self) -> &Transcript<G::Move, P> {
        &self.in_progress
    }

    pub fn history(&self) -> &History<G::Kind, G::Move, G::Utility, P> {
        &self.history
    }

    pub fn score(&self) -> Payoff<G::Utility, P> {
        self.history.score()
    }
}

// Unfortunately, we have to manually implement the following traits because Rust can't determine
// proper trait bounds in the presence of associated types.

impl<G: Game<P>, const P: usize> Clone for Context<G, P> {
    fn clone(&self) -> Self {
        Context {
            current_player: self.current_player,
            game_state: self.game_state.clone(),
            in_progress: self.in_progress.clone(),
            history: self.history.clone(),
        }
    }
}

impl<G: Game<P>, const P: usize> Debug for Context<G, P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Context")
            .field("current_player", &self.current_player)
            .field("game_state", &self.game_state)
            .field("in_progress", &self.in_progress)
            .field("history", &self.history)
            .finish()
    }
}

impl<G: Game<P>, const P: usize> PartialEq for Context<G, P> {
    fn eq(&self, other: &Self) -> bool {
        self.current_player == other.current_player
            && self.game_state == other.game_state
            && self.in_progress == other.in_progress
            && self.history == other.history
    }
}
