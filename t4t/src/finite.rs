use crate::{Game, PlayerIndex, PossibleMoves};

/// In a finite game, there is a finite set of moves available on each turn.
///
/// Note that there are two definitions of "finite game" in game theory. The more common definition
/// is a game with a finite number of turns, which is not the definition used here.
pub trait Finite<const P: usize>: Game<P> {
    /// Get an iterator over the moves available to the given player from the given game state.
    fn possible_moves(
        &self,
        player: PlayerIndex<P>,
        state: &Self::State,
    ) -> PossibleMoves<'_, Self::Move>;
}
