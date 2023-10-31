use std::fmt::Debug;

use crate::{Action, Context, Kind, Move, Payoff, Profile, Seq, Sim, Utility};

pub trait State: Clone + Debug + PartialEq {}
impl<T: Clone + Debug + PartialEq> State for T {}

/// A root trait that all games implement, mostly used for its associated types.
pub trait Game<const P: usize>: Sized {
    /// Type that indicates whether the game is simultaneous ([`Sim`]) or seqeuntial ([`Seq`]).
    type Kind: Kind;

    /// The type of intermediate state used to support the execution of a single iteration of the
    /// game.
    ///
    /// For [simultaneous](Sim) games this will be `()` since no intermediate state is required.
    type State: State;

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
pub trait Simultaneous<const P: usize>: Game<P, Kind = Sim> {
    /// Get the payoff for the given strategy profile in the given context.
    ///
    /// This method may return an arbitrary payoff if given a profile containing an invalid move.
    ///
    /// Typically, the context will be irrelevant to computing the payoff of a simultaneous game.
    /// However, it is provided as an argument to enable defining games that vary over time and
    /// other potential unforeseen use cases.
    fn payoff_in_context(
        &self,
        profile: Profile<Self::Move, P>,
        context: &Context<Self, P>,
    ) -> Payoff<Self::Utility, P>;
}

/// A trait implemented by sequential games, enabling them to be played via the
/// [`Playable`](crate::Playable) trait.
pub trait Sequential<const P: usize>: Game<P, Kind = Seq> {
    /// Get the next action for the given game state in the given context.
    ///
    /// This method may return an arbitrary action if called after a game has ended. However, a
    /// a good choice is probably to just keep returning the [`Action::Payoff`] action that ended
    /// the game.
    ///
    /// Typically, only the state argument will be used for determining the next action in the game.
    /// In particular, whose turn it is should be tracked in the game's state and implementers
    /// should *not* rely on [`Context::get_current_player`] since the context's current player is
    /// only set while executing a player's turn, not in between turns.
    ///
    /// However, the context is provided as an argument to enable defining games that vary over
    /// time and other potential unforeseen use cases.
    fn initial_action(&self) -> Action<Self::State, Self::Move, Self::Utility, P>;
}
