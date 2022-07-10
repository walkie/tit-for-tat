use dyn_clone::DynClone;
use std::fmt::Debug;
use std::hash::Hash;

/// A trait that collects the trait requirements of moves.
///
/// A blanket implementation covers all types that meet the requirements, so this trait should not
/// be implemented directly.
pub trait IsMove: Copy + Debug + Eq + Hash + 'static {}
impl<T: Copy + Debug + Eq + Hash + 'static> IsMove for T {}

/// An iterator that can be cloned, enabling it to be used multiple times.
///
/// A blanket implementation covers all types that meet the requirements, so this trait should not
/// be implemented directly.
trait CloneableIterator<I>: DynClone + Iterator<Item = I> {}
impl<I, T: DynClone + Iterator<Item = I>> CloneableIterator<I> for T {}

dyn_clone::clone_trait_object!(<I> CloneableIterator<I>);

/// An iterator over available moves in a [`Finite`](crate::game::Finite) or
/// [`FiniteSimultaneous`](crate::game::FiniteSimultaneous) game.
#[derive(Clone)]
pub struct MoveIter<'game, Move> {
    iter: Box<dyn CloneableIterator<Move> + 'game>,
}

impl<'game, Move> MoveIter<'game, Move> {
    /// Construct a new move iterator.
    pub fn new(iter: impl Clone + Iterator<Item = Move> + 'game) -> Self {
        MoveIter {
            iter: Box::new(iter),
        }
    }
}

impl<'game, Move> Iterator for MoveIter<'game, Move> {
    type Item = Move;
    fn next(&mut self) -> Option<Move> {
        self.iter.next()
    }
}
