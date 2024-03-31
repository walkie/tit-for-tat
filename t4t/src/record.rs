use crate::summary::Summary;
use crate::{Move, PerPlayer, PlayerIndex, Plies, Transcript};

/// A record of moves played during a game.
///
/// The canonical implementations of this trait are:
/// - [`Profile`](crate::Profile) for simultaneous games
/// - [`Transcript`](crate::Transcript) for sequential games
/// - [`History`](crate::History) for repeated games
pub trait Record<M: Move, const P: usize> {
    /// An iterator over the played moves in this record.
    ///
    /// A [ply](https://en.wikipedia.org/wiki/Ply_(game_theory)) typically refers only to a move
    /// played in a sequential game. For records of simultaneous games this iterator will return
    /// the move played by each player in order of their player index.
    fn plies(&self) -> Plies<M, P>;

    /// A summary of the number of moves in this record.
    fn summary(&self) -> Summary<P>;

    /// Get the moves played in the form of a transcript.
    fn to_transcript(&self) -> Transcript<M, P> {
        self.plies().into_transcript()
    }

    /// An iterator over all moves by chance.
    fn played_moves_by_chance(&self) -> PlayedMoves<M> {
        let move_iter = self
            .plies()
            .filter(move |ply| ply.player == None)
            .map(|ply| ply.the_move);
        PlayedMoves::from_iter(move_iter)
    }

    /// An iterator over all moves by a particular player.
    fn played_moves_by_player(&self, player: PlayerIndex<P>) -> PlayedMoves<M> {
        let move_iter = self
            .plies()
            .filter(move |ply| ply.player == Some(player))
            .map(|ply| ply.the_move);
        PlayedMoves::from_iter(move_iter)
    }

    /// Iterators over the moves by each player.
    fn played_moves_per_player(&self) -> PerPlayer<PlayedMoves<M>, P> {
        PerPlayer::generate(|player| self.played_moves_by_player(player))
    }
}

/// An iterator over the moves played in a game.
///
/// This iterator is double-ended, so it can be traversed forward (starting from the beginning of
/// the game) or backward (starting from the most recent move).
pub struct PlayedMoves<'a, M> {
    iterator: Box<dyn DoubleEndedIterator<Item=M> + 'a>,
}

impl<'a, M: Move> PlayedMoves<'a, M> {
    pub fn empty() -> Self {
        PlayedMoves {
            iterator: Box::new(std::iter::empty()),
        }
    }

    pub fn from_move(the_move: M) -> Self {
        PlayedMoves {
            iterator: Box::new(std::iter::once(the_move)),
        }
    }

    /// Construct a new played move iterator from a double-ended iterator of moves.
    pub fn from_iter(iterator: impl DoubleEndedIterator<Item=M> + 'a) -> Self {
        PlayedMoves {
            iterator: Box::new(iterator),
        }
    }

    /// Construct a new played move iterator from a vector of moves.
    pub fn from_vec(moves: Vec<M>) -> Self {
        PlayedMoves::from_iter(moves.into_iter())
    }
}

impl<'a, M> Iterator for PlayedMoves<'a, M> {
    type Item = M;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next()
    }
}

impl<'a, M> DoubleEndedIterator for PlayedMoves<'a, M> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iterator.next_back()
    }
}
