use std::fmt::{Debug, Display};

use crate::{Move, PlayerIndex, State};

/// A result while playing a game. Either a value of type `T` or an [`InvalidMove`] error.
pub type PlayResult<T, S, M, const P: usize> = Result<T, PlayError<S, M, P>>;

/// An error that occurred while playing a game.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct PlayError<S, M, const P: usize> {
    /// The game state when the error occurred.
    pub state: S,

    /// The kind of error that occurred.
    pub kind: PlayErrorKind<M, P>,
}

/// The kind of error that occurred.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum PlayErrorKind<M, const P: usize> {
    /// A player played an invalid move.
    InvalidMove(PlayerIndex<P>, M),

    /// There is no valid move available for the player to play.
    ///
    /// This is likely an error in the game's construction.
    NoValidMove(PlayerIndex<P>),
}

impl<S, M, const P: usize> PlayError<S, M, P> {
    /// Construct a new error.
    pub fn new(state: S, kind: PlayErrorKind<M, P>) -> Self {
        PlayError { state, kind }
    }

    /// Create a new invalid move error.
    pub fn invalid_move(state: S, player: PlayerIndex<P>, the_move: M) -> Self {
        PlayError {
            state,
            kind: PlayErrorKind::InvalidMove(player, the_move),
        }
    }

    /// Create a new no-valid-move error.
    pub fn no_valid_move(state: S, player: PlayerIndex<P>) -> Self {
        PlayError {
            state,
            kind: PlayErrorKind::NoValidMove(player),
        }
    }
}

impl<S, M: Move, const P: usize> Display for PlayError<S, M, P> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self.kind {
            PlayErrorKind::InvalidMove(player, the_move) => {
                format!("player {} played an invalid move: {:?}", player, the_move)
            }
            PlayErrorKind::NoValidMove(player) => format!("player {} has no valid moves", player),
        };
        write!(fmt, "{}", msg)
    }
}

impl<S: State, M: Move, const P: usize> std::error::Error for PlayError<S, M, P> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
