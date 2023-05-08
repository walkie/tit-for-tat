use crate::game::Game;
use crate::history::{GameRecord, History};
use crate::payoff::Payoff;
use crate::per_player::PlayerIndex;
use crate::transcript::Transcript;

/// The strategic context in which a player makes a move.
///
/// This type includes all of the information, besides the definition of the stage game, that a
/// repeated game strategy may use to compute its next move. It inclues the history of past games
/// played, as well as the game state and a transcript of moves played so far in the current game
/// iteration.
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
