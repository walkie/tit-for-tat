use std::fmt::Debug;

use crate::{Context, Kind, Move, Payoff, Profile, Seq, Sim, Utility};

/// A root trait that all games implement, mostly used for its associated types.
pub trait Game<const P: usize>: Sized {
    /// Type that indicates whether the game is simultaneous ([`Sim`]) or
    /// sequential ([`Seq`]).
    type Kind: Kind;

    /// The type of intermediate state used to support the execution of a single iteration of the
    /// game.
    ///
    /// For [simultaneous](crate::Simultaneous) and [normal-form](crate::Normal) games, this will
    /// be `()`, since no intermediate state is required. For [extensive-form](Extensive) games,
    /// the state will be the location in the game tree. For state-based games, the state type
    /// will be whatever state is used to define the game.
    type State: Clone + Debug + PartialEq;

    /// The type of moves played by players in this game.
    type Move: Move;

    /// The type of utility values awarded to each player at the end of the game.
    type Utility: Utility;

    /// The initial game state.
    fn initial_state(&self) -> Self::State;

    /// Is this a valid move in the given context?
    fn is_valid_move_in_context(&self, context: &Context<Self, P>, the_move: Self::Move) -> bool;

    /// Is this a sequential game?
    fn is_sequential(&self) -> bool {
        Self::Kind::is_sequential()
    }

    /// Is this a simultaneous game?
    fn is_simultaneous(&self) -> bool {
        Self::Kind::is_simultaneous()
    }

    /// The number of players this game is for.
    fn num_players(&self) -> usize {
        P
    }
}

/// A trait implemented by simultaneous games, enabling them to be played via the
/// [`Playable`](crate::Playable) trait.
pub trait Simultaneous<const P: usize>: Game<P, Kind = Sim, State = ()> {
    /// Get the payoff for the given strategy profile in the given context.
    ///
    /// This method may return an arbitrary payoff if given an
    /// [invalid profile](Simultaneous::is_valid_profile).
    fn payoff_in_context(
        &self,
        context: &Context<Self, P>,
        profile: Profile<Self::Move, P>,
    ) -> Payoff<Self::Utility, P>;
}

/// A trait implemented by sequential games, enabling them to be played via the
/// [`Playable`](crate::Playable) trait.
pub trait Sequential<const P: usize>: Game<P, Kind = Seq> {}
