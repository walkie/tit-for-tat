use thiserror::Error;

use crate::PlayerIndex;

/// An error while playing a game.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq, Hash)]
pub enum Error<M, const P: usize> {
    /// A player played an invalid move.
    InvalidMove(PlayerIndex<P>, M),

    /// An apparently valid move did not produce the next intermediate state the game. This is
    /// likely an error in the construction of the game.
    MalformedGame(M),
}
