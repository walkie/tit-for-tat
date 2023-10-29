use crate::internal::Sealed;
use crate::{Move, Profile, Transcript};
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;

/// The basic structure of a game. How players take turns, either simultaneously or sequentially.
///
/// This is a sealed trait with two instances corresponding to the two kinds of games:
/// - [`Sim`] for simultaneous games
/// - [`Seq<S>`](Seq) for sequential games with intermediate state `S`
pub trait Kind: Sealed + Clone + Copy + Debug + Eq + PartialEq + Hash {
    /// The type used to represent a record of moves played in this kind of game.
    ///
    /// This will be:
    /// - [`Profile`] for [simultaneous](Sim) games
    /// - [`Transcript`] for [sequential](Seq) games
    type Record<M: Move, const P: usize>: Clone + Debug + PartialEq + Hash + Sized;

    /// The type of intermediate state used to support the execution of a single iteration of the
    /// game.
    ///
    /// For [simultaneous](Sim) games this will be `()` since no intermediate state is required.
    type State: Clone + Debug + PartialEq;

    /// Is this a sequential game?
    fn is_sequential() -> bool;

    /// Is this a simultaneous game?
    fn is_simultaneous() -> bool;
}

/// Type marker for simultaneous games.
///
/// This type is never instantiated.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Sim {
    _private: (), // make the Sim type abstract
}

impl Sealed for Sim {}

impl Kind for Sim {
    type Record<M: Move, const P: usize> = Profile<M, P>;

    type State = ();

    fn is_sequential() -> bool {
        false
    }

    fn is_simultaneous() -> bool {
        true
    }
}

/// Type marker for sequential games with intermediate state `S`.
///
/// This type is never instantiated.
#[derive(Debug)]
pub struct Seq<S> {
    _state: PhantomData<S>,
}

impl<S> Sealed for Seq<S> {}

impl<S: Clone + Debug + PartialEq> Kind for Seq<S> {
    type Record<M: Move, const P: usize> = Transcript<M, P>;

    type State = S;

    fn is_sequential() -> bool {
        true
    }

    fn is_simultaneous() -> bool {
        false
    }
}

// Unfortunately, we have to manually implement the following traits because Rust can't determine
// proper trait bounds for phantom types.

impl<S> Clone for Seq<S> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<S> Copy for Seq<S> {}

impl<S> PartialEq for Seq<S> {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl<S> Eq for Seq<S> {}

impl<S> Hash for Seq<S> {
    fn hash<H: std::hash::Hasher>(&self, _state: &mut H) {}
}
