use std::error::Error;
use std::fmt::{Debug, Display};

use crate::{Context, Game, PlayerIndex};

/// A specialization of the standard [`Result`] type for handling failures while playing a game.
///
/// Either a value of type `T` on success or an error in the context that it occurred on failure.
pub type PlayResult<T, G, const P: usize> = Result<T, PlayErrorInContext<G, P>>;

/// An error in the game execution context in which it occurred.
pub struct PlayErrorInContext<G: Game<P>, const P: usize> {
    pub context: Context<G, P>,
    pub error: PlayError<G, P>,
}

impl<G: Game<P>, const P: usize> PlayErrorInContext<G, P> {
    pub fn new(context: Context<G, P>, error: PlayError<G, P>) -> Self {
        PlayErrorInContext { context, error }
    }
}

/// An error while playing a game.
pub enum PlayError<G: Game<P>, const P: usize> {
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

impl<G: Game<P>, const P: usize> Clone for PlayError<G, P> {
    fn clone(&self) -> Self {
        match self {
            PlayError::InvalidMove(player, the_move) => PlayError::InvalidMove(*player, *the_move),
            PlayError::NoNextState(the_move) => PlayError::NoNextState(*the_move),
        }
    }
}

impl<G: Game<P>, const P: usize> Debug for PlayError<G, P> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayError::InvalidMove(player, the_move) => fmt
                .debug_tuple("InvalidMove")
                .field(player)
                .field(the_move)
                .finish(),
            PlayError::NoNextState(the_move) => {
                fmt.debug_tuple("NoNextState").field(the_move).finish()
            }
        }
    }
}

impl<G: Game<P>, const P: usize> PartialEq for PlayError<G, P> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (PlayError::InvalidMove(p1, m1), PlayError::InvalidMove(p2, m2)) => {
                p1 == p2 && m1 == m2
            }
            (PlayError::NoNextState(m1), PlayError::NoNextState(m2)) => m1 == m2,
            _ => false,
        }
    }
}

impl<G: Game<P>, const P: usize> Clone for PlayErrorInContext<G, P> {
    fn clone(&self) -> Self {
        PlayErrorInContext {
            context: self.context.clone(),
            error: self.error.clone(),
        }
    }
}

impl<G: Game<P>, const P: usize> Debug for PlayErrorInContext<G, P> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("PlayErrorInContext")
            .field("context", &self.context)
            .field("error", &self.error)
            .finish()
    }
}

impl<G: Game<P>, const P: usize> Display for PlayErrorInContext<G, P> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self.error {
            PlayError::InvalidMove(player, the_move) => {
                format!("player {} played an invalid move: {:?}", player, the_move)
            }
            PlayError::NoNextState(the_move) => {
                format!("no next state for apparently valid move: {:?}", the_move)
            }
        };
        write!(fmt, "{}", msg)
    }
}

impl<G: Game<P>, const P: usize> Error for PlayErrorInContext<G, P> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl<G: Game<P>, const P: usize> PartialEq for PlayErrorInContext<G, P> {
    fn eq(&self, other: &Self) -> bool {
        self.context == other.context && self.error == other.error
    }
}
