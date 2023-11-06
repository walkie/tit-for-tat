use crate::{PlayerIndex, Transcript};

/// A [ply](https://en.wikipedia.org/wiki/Ply_(game_theory)) is a single move played during a
/// sequential game.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Ply<M, const P: usize> {
    /// The player that played the move, or `None` if it was a move of chance.
    pub player: Option<PlayerIndex<P>>,
    /// The move that was played.
    pub the_move: M,
}

impl<M, const P: usize> Ply<M, P> {
    /// Construct a new played move.
    pub fn new(player: Option<PlayerIndex<P>>, the_move: M) -> Self {
        Ply { player, the_move }
    }

    /// Construct a move played by the given player.
    pub fn player(player: PlayerIndex<P>, the_move: M) -> Self {
        Ply::new(Some(player), the_move)
    }

    /// Construct a move played by chance.
    pub fn chance(the_move: M) -> Self {
        Ply::new(None, the_move)
    }

    /// Was this move played by a player (and not chance)?
    pub fn is_player(&self) -> bool {
        self.player.is_some()
    }

    /// Was this move played by chance?
    pub fn is_chance(&self) -> bool {
        self.player.is_none()
    }
}

/// An iterator over the plies in a game.
pub struct PlyIter<'a, M, const P: usize> {
    iter: Box<dyn Iterator<Item = Ply<M, P>> + 'a>,
}

impl<'a, M, const P: usize> PlyIter<'a, M, P> {
    /// Construct a new ply iterator.
    pub fn new(iter: impl Iterator<Item = Ply<M, P>> + 'a) -> Self {
        PlyIter {
            iter: Box::new(iter),
        }
    }

    /// Collect the plies in this iterator into a transcript.
    pub fn to_transcript(&self) -> Transcript<M, P> {
        Transcript::from_played_moves(self.iter.collect())
    }
}

impl<'a, M, const P: usize> Iterator for PlyIter<'a, M, P> {
    type Item = Ply<M, P>;
    fn next(&mut self) -> Option<Ply<M, P>> {
        self.iter.next()
    }
}
