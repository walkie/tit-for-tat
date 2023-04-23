use crate::history::History;
use crate::moves::IsMove;
use crate::payoff::{IsUtility, Payoff};
use crate::per_player::PlayerIndex;
use crate::player::Players;
use crate::record::Record;
use crate::transcript::Transcript;

/// An interface for playing games.
pub trait Playable<const N: usize>: Sized {
    /// The type of moves played by players in this game.
    type Move: IsMove;

    /// The type of utility values awarded to each player at the end of the game.
    type Utility: IsUtility;

    /// The type of intermediate state required to support the execution of a single iteration of
    /// this game.
    type GameState;

    /// The initial state of the game.
    fn initial_state(&self) -> Self::GameState;

    /// Play one iteration of the game and update the execution state accordingly. Returns the
    /// record of this game iteration, if successful.
    ///
    /// # Note to implementors
    ///
    /// In addition to returning the completed game record, this method should add the record to
    /// the execution state using [PlayState::add_record]. For sequential games, it will also need
    /// to update the current game's transcript using [PlayState::add_move] after getting and
    /// executing each player's move.
    fn play(
        &self,
        players: &mut Players<Self, N>,
        state: &mut PlayState<Self, N>,
    ) -> PlayResult<Record<Self::Move, Self::Utility, N>, Self::Move, N>;

    /// Play a game once with the given players, starting from the initial state.
    fn play_once(
        &self,
        players: &mut Players<Self, N>,
    ) -> PlayResult<Record<Self::Move, Self::Utility, N>, Self::Move, N> {
        let mut state = PlayState::new(self);
        self.play(players, &mut state)
    }

    /// Play a given number of iterations of a game with the given players, starting from the
    /// initial state. Returns the final execution state, if successful.
    fn play_repeatedly(
        &self,
        players: &mut Players<Self, N>,
        iterations: u32,
    ) -> PlayResult<PlayState<Self, N>, Self::Move, N> {
        let mut state = PlayState::new(self);
        for _ in 0..iterations {
            match self.play(players, &mut state) {
                Ok(record) => state.completed.add_record(record),
                Err(err) => return Err(err),
            }
        }
        Ok(state)
    }
}

/// Result of playing a game. Either a record of the completed game or an error.
pub type PlayResult<T, Move, const N: usize> = Result<T, PlayError<Move, N>>;

/// An error during game execution.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum PlayError<Move, const N: usize> {
    /// A player played an invalid move.
    InvalidMove(PlayerIndex<N>, Move),

    /// An apparently valid move did not produce the next node in the game tree. This is likely an
    /// error in the construction of the game.
    MalformedGame(Move),
}

/// A state tracking all aspects of repeated game execution.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct PlayState<Game: Playable<N>, const N: usize> {
    current_player: Option<PlayerIndex<N>>,
    game_state: Game::GameState,
    in_progress: Transcript<Game::Move, N>,
    completed: History<Game::Move, Game::Utility, N>,
}

impl<Game: Playable<N>, const N: usize> PlayState<Game, N> {
    pub fn new(game: &Game) -> Self {
        PlayState {
            current_player: None,
            game_state: game.initial_state(),
            in_progress: Transcript::new(),
            completed: History::new(),
        }
    }

    pub fn add_record(&mut self, record: Record<Game::Move, Game::Utility, N>) {
        self.completed.add_record(record);
    }
}
