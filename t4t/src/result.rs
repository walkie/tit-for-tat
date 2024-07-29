use std::fmt::{Debug, Display};

use crate::{Move, PlayerIndex, State};

/// A result while playing a game. Either a value of type `T` or an [`InvalidMove`] error.
pub type PlayResult<T, S, M, const P: usize> = Result<T, InvalidMove<S, M, P>>;

/// An error caused by a player playing an invalid move during a game.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct InvalidMove<S, M, const P: usize> {
    /// The game state at the point of the error.
    pub state: S,
    /// The player that made the invalid move.
    pub player: PlayerIndex<P>,
    /// The invalid move.
    pub the_move: M,
}

impl<S, M, const P: usize> InvalidMove<S, M, P> {
    /// Create a new invalid move error.
    pub fn new(state: S, player: PlayerIndex<P>, the_move: M) -> Self {
        InvalidMove {
            state,
            player,
            the_move,
        }
    }
}

impl<S, M: Move, const P: usize> Display for InvalidMove<S, M, P> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = format!(
            "player {} played an invalid move: {:?}",
            self.player, self.the_move
        );
        write!(fmt, "{}", msg)
    }
}

impl<S: State, M: Move, const P: usize> std::error::Error for InvalidMove<S, M, P> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
