use dyn_clone::DynClone;
use std::fmt::Debug;
use std::hash::Hash;

/// A trait that collects the trait requirements of moves.
///
/// A blanket implementation covers all types that meet the requirements, so this trait should not
/// be implemented directly.
pub trait IsMove: Copy + Debug + Eq + Hash + Sized + 'static {}
impl<T: Copy + Debug + Eq + Hash + 'static> IsMove for T {}

/// An iterator over available moves in a game with a finite move set.
#[derive(Clone)]
pub struct MoveIter<'a, Move> {
    iter: Box<dyn CloneableIterator<Move> + 'a>,
}

/// An iterator that can be cloned, enabling it to be used multiple times.
///
/// A blanket implementation covers all types that meet the requirements, so this trait should not
/// be implemented directly.
trait CloneableIterator<I>: DynClone + Iterator<Item = I> {}
impl<I, T: DynClone + Iterator<Item = I>> CloneableIterator<I> for T {}

dyn_clone::clone_trait_object!(<I> CloneableIterator<I>);

impl<'a, Move> MoveIter<'a, Move> {
    /// Construct a new move iterator.
    pub fn new(iter: impl Clone + Iterator<Item = Move> + 'a) -> Self {
        MoveIter {
            iter: Box::new(iter),
        }
    }
}

impl<'a, Move> Iterator for MoveIter<'a, Move> {
    type Item = Move;
    fn next(&mut self) -> Option<Move> {
        self.iter.next()
    }
}
