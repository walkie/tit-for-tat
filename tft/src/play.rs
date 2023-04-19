use crate::history::History;
use crate::moves::IsMove;
use crate::payoff::IsUtility;
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

    /// Play one iteration of the game, returning a record of the completed game or an error.
    fn play(&self, players: &mut Players<Self, N>, exec_state: &mut PlayState<Self, N>) -> PlayResult<Self::Move, Self::Utility, N>;

    fn play_once(&self, players: &mut Players<Self, N>) -> PlayResult<Self::Move, Self::Utility, N> {
        let mut state = PlayState::new(self);
        self.play(players, &mut state)
    }
}

/// Result of playing a game. Either a record of the completed game or an error.
pub type PlayResult<Move, Util, const N: usize> = Result<Record<Move, Util, N>, PlayError<Move, N>>;

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
}
