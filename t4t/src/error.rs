use std::fmt::{Debug, Display};
use std::rc::Rc;

use crate::{Game, PlayerIndex};

/// An error that occurred while playing a game and the current state when it occurred.
pub struct Error<G: Game<P>, const P: usize> {
    pub state: Rc<G::State>,
    pub kind: ErrorKind<G, P>,
}

impl<G: Game<P>, const P: usize> Error<G, P> {
    pub fn new(state: Rc<G::State>, kind: ErrorKind<G, P>) -> Self {
        Error { state, kind }
    }
}

/// The kind of error that occurred.
pub enum ErrorKind<G: Game<P>, const P: usize> {
    /// A player played an invalid move.
    // #[error("player P{} played an invalid move: {the_move}", .player.0)]
    InvalidMove(PlayerIndex<P>, G::Move),

    /// An apparently valid move did not produce the next intermediate state the game. This is
    /// likely an error in the construction of the game.
    // #[error("no next state for apparently valid move: {the_move}")]
    NoNextState(G::Move),
}

// Unfortunately, we have to manually implement the following traits because the derived instances
// assume that the game type `G` must also satisfy the traits, which isn't necessary.

impl<G: Game<P>, const P: usize> Clone for ErrorKind<G, P> {
    fn clone(&self) -> Self {
        match self {
            ErrorKind::InvalidMove(player, the_move) => ErrorKind::InvalidMove(*player, *the_move),
            ErrorKind::NoNextState(the_move) => ErrorKind::NoNextState(*the_move),
        }
    }
}

impl<G: Game<P>, const P: usize> Debug for ErrorKind<G, P> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::InvalidMove(player, the_move) => fmt
                .debug_tuple("InvalidMove")
                .field(player)
                .field(the_move)
                .finish(),
            ErrorKind::NoNextState(the_move) => {
                fmt.debug_tuple("NoNextState").field(the_move).finish()
            }
        }
    }
}

impl<G: Game<P>, const P: usize> PartialEq for ErrorKind<G, P> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ErrorKind::InvalidMove(p1, m1), ErrorKind::InvalidMove(p2, m2)) => {
                p1 == p2 && m1 == m2
            }
            (ErrorKind::NoNextState(m1), ErrorKind::NoNextState(m2)) => m1 == m2,
            _ => false,
        }
    }
}

impl<G: Game<P>, const P: usize> Clone for Error<G, P> {
    fn clone(&self) -> Self {
        Error {
            state: self.state.clone(),
            kind: self.kind.clone(),
        }
    }
}

impl<G: Game<P>, const P: usize> Debug for Error<G, P> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("Error")
            .field("state", &self.state)
            .field("kind", &self.kind)
            .finish()
    }
}

impl<G: Game<P>, const P: usize> Display for Error<G, P> {
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

impl<G: Game<P>, const P: usize> std::error::Error for Error<G, P> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl<G: Game<P>, const P: usize> PartialEq for Error<G, P> {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state && self.kind == other.kind
    }
}
