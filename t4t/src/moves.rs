use dyn_clone::DynClone;
use std::fmt::Debug;
use std::hash::Hash;

/// A trait that collects the trait requirements of moves.
///
/// A blanket implementation covers all types that meet the requirements, so this trait should not
/// be implemented directly.
pub trait Move: Copy + Debug + Eq + PartialEq + Hash + Sized + 'static {}

impl<T: Copy + Debug + Eq + Hash + 'static> Move for T {}

/// An iterator over the possible moves at a particular point in a game with a finite move set.
///
/// This iterator is cloneable, so it can be used multiple times.
#[derive(Clone)]
pub struct PossibleMoves<'a, M> {
    iterator: Box<dyn CloneableIterator<M> + 'a>,
}

impl<'a, M: Move> PossibleMoves<'a, M> {
    /// Construct a new possible move iterator from a cloneable iterator of moves.
    pub fn from_iter(iterator: impl Clone + Iterator<Item = M> + 'a) -> Self {
        PossibleMoves {
            iterator: Box::new(iterator),
        }
    }

    /// Construct a new possible move iterator from a vector of moves.
    pub fn from_vec(moves: Vec<M>) -> Self {
        PossibleMoves::from_iter(moves.into_iter())
    }
}

impl<'a, M> Iterator for PossibleMoves<'a, M> {
    type Item = M;
    fn next(&mut self) -> Option<M> {
        self.iterator.next()
    }
}

/// An iterator that can be cloned, enabling it to be used multiple times.
///
/// A blanket implementation covers all types that meet the requirements, so this trait should not
/// be implemented directly.
trait CloneableIterator<T>: DynClone + Iterator<Item = T> {}
impl<T, I: DynClone + Iterator<Item = T>> CloneableIterator<T> for I {}

dyn_clone::clone_trait_object!(<I> CloneableIterator<I>);
