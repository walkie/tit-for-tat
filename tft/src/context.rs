use crate::seq::Transcript;
use crate::{History, Move, Payoff, PlayerIndex, Record, Utility};

/// The strategic context in which a player makes a move during a repeated game.
///
/// This type includes all information, besides the definition of the stage game, that a strategy
/// for a repeated game may use to compute its next move. It includes the history of past games
/// played, the game state of the current iteration, and (for sequential games) the transcript of
/// moves played so far in the current iteration.
#[derive(Clone, Debug, PartialEq)]
pub struct Context<S, M: Move, U: Utility, R: Record<U, P>, const P: usize> {
    current_player: Option<PlayerIndex<P>>,
    game_state: Option<S>,
    in_progress: Transcript<M, P>,
    history: History<U, R, P>,
}

impl<S, M: Move, U: Utility, R: Record<U, P>, const P: usize> Context<S, M, U, R, P> {
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
        if let Some(state) = Option::take(&mut self.game_state) {
            self.game_state = update(state);
        }
    }

    pub fn complete(&mut self, record: R) -> &R {
        self.history.add(record)
    }

    pub fn current_player(&self) -> Option<PlayerIndex<P>> {
        self.current_player
    }

    pub fn game_state(&self) -> Option<&S> {
        self.game_state.as_ref()
    }

    pub fn in_progress(&self) -> &Transcript<M, P> {
        &self.in_progress
    }

    pub fn history(&self) -> &History<U, R, P> {
        &self.history
    }

    pub fn score(&self) -> Payoff<U, P> {
        self.history.score()
    }
}
