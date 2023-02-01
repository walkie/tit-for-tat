use crate::moves::IsMove;

/// A strategy is a function from the an intermediate game state to a move.
///
/// # Type variables
///
/// - `Move` -- The type of moves yielded by this strategy.
/// - `State` -- The type of the game state used to compute the next move.
pub trait Strategy<Move: IsMove, State> {
    /// Get the next move to play given a particular game state.
    fn next_move(&mut self, state: &State) -> Move;
}
