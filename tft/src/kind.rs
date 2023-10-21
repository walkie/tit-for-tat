use crate::internal::Sealed;
use crate::{Move, Profile, Transcript};
use std::fmt::Debug;
use std::hash::Hash;

/// The basic structure of a game. How players take turns, either simultaneously or sequentially.
///
/// This is a sealed trait with exactly two instances corresponding to the two kinds of games:
/// - [`Sim`](Sim) for simultaneous games
/// - [`Seq`](Seq) for sequential games
pub trait Kind: Sealed {
    /// The type used to represent a record of moves played in this kind of game.
    ///
    /// This will be:
    /// - [`Profile`](crate::Profile) for simultaneous games
    /// - [`Transcript`](crate::Transcript) for sequential games
    type Record<M: Move, const P: usize>: Clone + Debug + PartialEq + Hash + Sized;

    /// Is this a sequential game?
    fn is_sequential() -> bool;

    /// Is this a simultaneous game?
    fn is_simultaneous() -> bool;
}

/// Type marker for sequential games.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Seq;

impl Sealed for Seq {}

impl Kind for Seq {
    type Record<M: Move, const P: usize> = Transcript<M, P>;

    fn is_sequential() -> bool {
        true
    }

    fn is_simultaneous() -> bool {
        false
    }
}

/// Type marker for simultaneous games.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Sim;

impl Sealed for Sim {}

impl Kind for Sim {
    type Record<M: Move, const P: usize> = Profile<M, P>;

    fn is_sequential() -> bool {
        false
    }

    fn is_simultaneous() -> bool {
        true
    }
}
