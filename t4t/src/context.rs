use std::fmt::Debug;
use std::rc::Rc;

use crate::{Game, Payoff, PlayerIndex, Transcript};

/// The strategic context in which a player makes a move during a game.
///
/// This type includes all information, besides the definition of the stage game, that a strategy
/// may use to compute its next move. It includes the player's index, the player's view of the game
/// state, a transcript of actions so far, and the current score.
#[derive(Clone, Debug, PartialEq)]
pub struct Context<G: Game<P>, const P: usize> {
    player_index: PlayerIndex<P>,
    state_view: G::StateView,
    transcript: Rc<Transcript<G::Move, P>>,
    score: Payoff<G::Utility, P>,
}

// pub fn complete(
//     &mut self,
//     outcome: Outcome<G::Kind, G::Move, G::Utility, P>,
// ) -> &Outcome<G::Kind, G::Move, G::Utility, P> {
//     self.history.add(outcome)
// }

// pub fn current_player(&self) -> Option<PlayerIndex<P>> {
//     self.current_player
// }

// pub fn game_state(&self) -> Option<&G::State> {
//     self.game_state.as_ref()
// }

// pub fn in_progress(&self) -> &Transcript<G::Move, P> {
//     &self.in_progress
// }

// pub fn history(&self) -> &History<G::Kind, G::Move, G::Utility, P> {
//     &self.history
// }

// pub fn score(&self) -> Payoff<G::Utility, P> {
//     self.history.score()
// }
