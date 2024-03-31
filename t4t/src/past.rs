/// An iterator over past events while playing a game.
///
/// The iterator is double-ended, so it can be traversed forward (starting from the beginning of
/// the game) or backward (starting from the most recent event).
pub struct Past<'a, T> {
    length: usize,
    iterator: Box<dyn DoubleEndedIterator<Item = T> + 'a>,
}

impl<'a, T> Past<'a, T> {
    /// Construct a new iterator from an existing double-ended iterator.
    pub fn from_iter(length: usize, iterator: impl DoubleEndedIterator<Item = T> + 'a) -> Self {
        Past {
            length,
            iterator: Box::new(iterator),
        }
    }

    /// Construct a new iterator from a vector of events.
    pub fn from_vec(events: Vec<T>) -> Self
    where
        T: 'a,
    {
        Past::from_iter(events.len(), events.into_iter())
    }
}

impl<'a, T> Iterator for Past<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.iterator.next()
    }
}

impl<'a, T> DoubleEndedIterator for Past<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iterator.next_back()
    }
}

impl<'a, T> ExactSizeIterator for Past<'a, T> {
    fn len(&self) -> usize {
        self.length
    }
}
