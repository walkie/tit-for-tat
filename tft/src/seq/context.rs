use crate::moves::Move;
use crate::payoff::{Payoff, Utility};
use crate::per_player::PlayerIndex;
use crate::seq::outcome::Outcome;
use crate::seq::transcript::Transcript;

pub type History<M, U, const P: usize> = crate::History<U, Outcome<M, U, P>, P>;

/// The strategic context in which a player makes a move during a repeated sequential game.
///
/// This type includes all of the information, besides the definition of the stage game, that a
/// repeated sequential game strategy may use to compute its next move. It inclues the history of
/// past games played, the game state of the current iteration, and a transcript of moves played
/// so far in the current iteration.
#[derive(Clone, Debug, PartialEq)]
pub struct Context<S, M: Move, U: Utility, const P: usize> {
    current_player: Option<PlayerIndex<P>>,
    game_state: Option<S>,
    in_progress: Transcript<M, P>,
    history: History<M, U, P>,
}

impl<S, M: Move, U: Utility, const P: usize> Context<S, M, U, P> {
    pub fn new(init_state: S) -> Self {
        Context {
            current_player: None,
            game_state: Some(init_state),
            in_progress: Transcript::new(),
            history: History::new(),
        }
    }

    pub fn set_current_player(&mut self, player: PlayerIndex<P>) {
        self.current_player = Some(player);
    }

    pub fn unset_current_player(&mut self) {
        self.current_player = None;
    }

    pub fn set_game_state(&mut self, state: S) {
        self.game_state = Some(state);
    }

    pub fn update_game_state<F>(&mut self, update: F)
    where
        F: FnOnce(S) -> Option<S>,
    {
        if let Some(state) = std::mem::replace(&mut self.game_state, None) {
            self.game_state = update(state);
        }
    }

    pub fn complete(
        &mut self,
        transcript: Transcript<M, P>,
        payoff: Payoff<U, P>,
    ) -> &Outcome<M, U, P> {
        self.history.add(Outcome::new(transcript, payoff))
    }

    pub fn current_player(&self) -> Option<PlayerIndex<P>> {
        self.current_player
    }

    pub fn game_state(&self) -> Option<&S> {
        self.game_state.as_ref()
    }

    pub fn history(&self) -> &History<M, U, P> {
        &self.history
    }

    pub fn score(&self) -> Payoff<U, P> {
        self.history.score()
    }
}
