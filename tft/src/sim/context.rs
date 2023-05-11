use crate::moves::Move;
use crate::payoff::{Payoff, Utility};
use crate::per_player::PlayerIndex;
use crate::sim::outcome::Outcome;
use crate::sim::profile::Profile;

/// For repeated simultaneous games, a history of previously played games.
pub type History<M, U, const P: usize> = crate::History<U, Outcome<M, U, P>, P>;

/// The strategic context in which a player makes a move during a repeated simultaneous game.
///
/// Contains the history of past games played.
#[derive(Clone, Debug, PartialEq)]
pub struct Context<M: Move, U: Utility, const P: usize> {
    current_player: Option<PlayerIndex<P>>,
    history: History<M, U, P>,
}

impl<M: Move, U: Utility, const P: usize> Context<M, U, P> {
    pub fn new() -> Self {
        Context {
            current_player: None,
            history: History::new(),
        }
    }

    pub fn set_current_player(&mut self, player: PlayerIndex<P>) {
        self.current_player = Some(player);
    }

    pub fn unset_current_player(&mut self) {
        self.current_player = None;
    }

    pub fn complete(&mut self, profile: Profile<M, P>, payoff: Payoff<U, P>) -> &Outcome<M, U, P> {
        self.history.add(Outcome::new(profile, payoff))
    }

    pub fn current_player(&self) -> Option<PlayerIndex<P>> {
        self.current_player
    }

    pub fn history(&self) -> &History<M, U, P> {
        &self.history
    }

    pub fn score(&self) -> Payoff<U, P> {
        self.history.score()
    }
}

impl<M: Move, U: Utility, const P: usize> Default for Context<M, U, P> {
    fn default() -> Self {
        Context::new()
    }
}
