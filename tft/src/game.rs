use crate::core::*;

pub mod ext;
pub mod norm;
pub mod seq;
pub mod sim;

/// The most general trait for games. Includes associated types and methods that all games must
/// support.
///
/// The const type variable `N` indicates the number of players this game is for.
pub trait Game<const N: usize> {
    /// The type of moves played during the game.
    type Move: IsMove;

    /// The type of utility value awarded to each player in the payoff at the end of the game.
    type Util: IsUtil;

    /// The number of players this game is for.
    fn num_players(&self) -> usize {
        N
    }
}
