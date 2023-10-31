use dyn_clone::DynClone;
use std::fmt::Debug;
use std::hash::Hash;

/// A trait that collects the trait requirements of moves.
///
/// A blanket implementation covers all types that meet the requirements, so this trait should not
/// be implemented directly.
pub trait Move: Copy + Debug + Eq + PartialEq + Hash + Sized + 'static {}

impl<T: Copy + Debug + Eq + Hash + 'static> Move for T {}

/// An iterator over available moves in a game with a finite move set.
#[derive(Clone)]
pub struct MoveIter<'a, M> {
    iter: Box<dyn CloneableIterator<M> + 'a>,
}

impl<'a, M> MoveIter<'a, M> {
    /// Construct a new move iterator.
    pub fn new(iter: impl Clone + Iterator<Item = M> + 'a) -> Self {
        MoveIter {
            iter: Box::new(iter),
        }
    }
}

impl<'a, M> Iterator for MoveIter<'a, M> {
    type Item = M;
    fn next(&mut self) -> Option<M> {
        self.iter.next()
    }
}

/// An iterator that can be cloned, enabling it to be used multiple times.
///
/// A blanket implementation covers all types that meet the requirements, so this trait should not
/// be implemented directly.
trait CloneableIterator<I>: DynClone + Iterator<Item = I> {}
impl<I, T: DynClone + Iterator<Item = I>> CloneableIterator<I> for T {}

dyn_clone::clone_trait_object!(<I> CloneableIterator<I>);
