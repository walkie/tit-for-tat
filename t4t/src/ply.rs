use crate::{Move, Past, PlayerIndex, Transcript};

/// A [ply](https://en.wikipedia.org/wiki/Ply_(game_theory)) is a single move played during a
/// sequential game.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Ply<M, const P: usize> {
    /// The player that played the move, or `None` if it was a move of chance.
    pub player: Option<PlayerIndex<P>>,
    /// The move that was played.
    pub the_move: M,
}

impl<M: Move, const P: usize> Ply<M, P> {
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
pub type Plies<'a, M, const P: usize> = Past<'a, Ply<M, P>>;

impl<'a, M: Move, const P: usize> Plies<'a, M, P> {
    /// Collect the plies in this iterator into a transcript.
    pub fn into_transcript(self) -> Transcript<M, P> {
        Transcript::from_plies(self)
    }
}
