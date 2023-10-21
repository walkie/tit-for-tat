use crate::internal::Sealed;
use crate::{Move, Profile, Transcript};
use std::fmt::Debug;
use std::hash::Hash;

pub trait Form: Sealed {
    type Record<M: Move, const P: usize>: Clone + Debug + PartialEq + Hash + Sized;

    fn is_sequential() -> bool;

    fn is_simultaneous() -> bool;
}

/// Type marker for sequential games.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Seq;

impl Sealed for Seq {}

impl Form for Seq {
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

impl Form for Sim {
    type Record<M: Move, const P: usize> = Profile<M, P>;

    fn is_sequential() -> bool {
        false
    }

    fn is_simultaneous() -> bool {
        true
    }
}
