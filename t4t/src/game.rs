use std::fmt::Debug;

use crate::{Move, Outcome, PlayerIndex, Utility};

/// A trait that collects the trait requirements of a game state.
///
/// A blanket implementation covers all types that meet the requirements, so this trait should not
/// be implemented directly.
pub trait State: Clone + Debug + PartialEq + Send + Sync + 'static {}
impl<T: Clone + Debug + PartialEq + Send + Sync + 'static> State for T {}

/// A root trait that all games implement.
///
/// This trait primarily serves two purposes:
/// - It specifies the trait requirements of all game types via its supertraits.
/// - It defines a set of associated types, common to all games, which simplifies type signatures
///   elsewhere.
///
/// Additionally, for games with hidden information, it defines the relationship between the
/// primary game state and the view of that state available to each player.
///
/// # Future work
/// There are two improvements planned for this trait, pending the stabilization of certain Rust
/// features:
///
/// - The const generic parameter `P` will be replaced by an associated constant when
///   [this Rust issue](https://github.com/rust-lang/rust/issues/60551) is resolved, allowing
///   associated constants to be used in const generics. This will simplify many type signatures
///   throughout the library.
///
/// - The associated type `View` will default to the associated type `State`, and the `state_view`
///   method will default to simply return the state unchanged, when
///   [associated type defaults](https://github.com/rust-lang/rust/issues/29661) are stabilized,
///   allowing implementers to skip specifying these for the common case of perfect-information
///   games.
pub trait Game<const P: usize>: Clone + Sized + Send + Sync {
    /// The type of moves played by players in this game.
    type Move: Move;

    /// The type of utility values awarded to each player at the end of the game.
    type Utility: Utility;

    /// The type of value produced by playing the game.
    /// - For simultaneous games: [`SimultaneousOutcome`](crate::SimultaneousOutcome)
    /// - For sequential games: [`SequentialOutcome`](crate::SequentialOutcome)
    /// - For repeated games: [`History`](crate::History)
    type Outcome: Outcome<Self::Move, Self::Utility, P>;

    /// The type of intermediate game state used during the execution of the game.
    type State: State;

    /// The type of the *view* of the intermediate game state presented to players.
    ///
    /// This may differ from [`State`] to support hidden information, that is, aspects of the game
    /// state that are not visible to players while making strategic decisions.
    type View: State;

    /// Produce a view of the game state for the given player.
    ///
    /// For perfect-information games, this should return the original state unchanged.
    fn state_view(&self, state: &Self::State, player: PlayerIndex<P>) -> Self::View;

    /// The number of players this game is for.
    fn num_players(&self) -> usize {
        P
    }
}
