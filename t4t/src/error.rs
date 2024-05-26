use std::fmt::{Debug, Display};
use std::sync::Arc;

use crate::{Move, PlayerIndex, State};

/// An error that occurred while playing a game and the current state when it occurred.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Error<S, M, const P: usize> {
    /// The current state at the point of the error.
    pub state: Arc<S>,
    /// The kind of error that occurred.
    pub kind: ErrorKind<M, P>,
}

impl<S, M, const P: usize> Error<S, M, P> {
    /// Construct a new gameplay error.
    pub fn new(state: Arc<S>, kind: ErrorKind<M, P>) -> Self {
        Error { state, kind }
    }
}

/// The kind of error that occurred.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum ErrorKind<M, const P: usize> {
    /// A player played an invalid move.
    // #[error("player P{} played an invalid move: {the_move}", .player.0)]
    InvalidMove(PlayerIndex<P>, M),

    /// An apparently valid move did not produce the next intermediate state the game. This is
    /// likely an error in the construction of the game.
    // #[error("no next state for apparently valid move: {the_move}")]
    NoNextState(M),
}

impl<S, M: Move, const P: usize> Display for Error<S, M, P> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self.kind {
            ErrorKind::InvalidMove(player, the_move) => {
                format!("player {} played an invalid move: {:?}", player, the_move)
            }
            ErrorKind::NoNextState(the_move) => {
                format!("no next state for apparently valid move: {:?}", the_move)
            }
        };
        write!(fmt, "{}", msg)
    }
}

impl<S: State, M: Move, const P: usize> std::error::Error for Error<S, M, P> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
