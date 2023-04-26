use crate::game::Game;
use crate::history::{GameRecord, History};
use crate::payoff::Payoff;
use crate::per_player::PlayerIndex;
use crate::transcript::Transcript;

/// Result of playing a game. Either a record of the completed game or an error.
pub type PlayResult<T, G, const P: usize> = Result<T, PlayError<G, P>>;

/// An error during game execution.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum PlayError<G: Game<P>, const P: usize> {
    /// A player played an invalid move.
    InvalidMove(PlayerIndex<P>, G::Move),

    /// An apparently valid move did not produce the next node in the game tree. This is likely an
    /// error in the construction of the game.
    MalformedGame(G::Move),
}

/// A state tracking all aspects of repeated game execution.
#[derive(Clone, Debug, PartialEq)]
pub struct PlayState<G: Game<P>, const P: usize> {
    current_player: Option<PlayerIndex<P>>,
    game_state: G::State,
    in_progress: Transcript<G::Move, P>,
    history: History<G, P>,
}

impl<G: Game<P>, const P: usize> PlayState<G, P> {
    pub fn new(game: &G) -> Self {
        PlayState {
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
