use crate::game::Game;
use crate::moves::Move;
use crate::payoff::{Payoff, Utility};
use crate::per_player::PlayerIndex;
use crate::seq::outcome::Outcome;
use crate::transcript::Transcript;

pub type History<M: Move, U: Utility, const P: usize> = crate::History<Outcome<M, U, P>, P>;

/// The strategic context in which a player makes a move during a repeated sequential game.
///
/// This type includes all of the information, besides the definition of the stage game, that a
/// repeated sequential game strategy may use to compute its next move. It inclues the history of
/// past games played, the game state of the current iteration, and a transcript of moves played
/// so far in the current iteration.
#[derive(Clone, Debug, PartialEq)]
pub struct Context<G: Game<P>, const P: usize> {
    current_player: Option<PlayerIndex<P>>,
    game_state: G::State,
    in_progress: Transcript<G::Move, P>,
    history: History<G, P>,
}

impl<G: Game<P>, const P: usize> Context<G, P> {
    pub fn new(game: &G) -> Self {
        Context {
            current_player: None,
            game_state: game.initial_state(),
            in_progress: Transcript::new(),
            history: History::new(),
        }
    }

    pub fn complete(
        &mut self,
        moves: G::MoveRecord,
        payoff: Payoff<G::Utility, P>,
    ) -> &GameRecord<G, P> {
        self.history.add(moves, payoff)
    }
}
