use derive_more::{AsMut, AsRef};
use std::ops::{Index, IndexMut};

use crate::{Move, PerPlayer, PlayedMoves, PlayerIndex, Plies, Ply, Record, Summary, Transcript};

/// A pure strategy profile for a simultaneous game: one move played by each player.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, AsMut, AsRef)]
pub struct Profile<M, const P: usize>(PerPlayer<M, P>);

impl<M, const P: usize> Profile<M, P> {
    /// Create a new profile from the given array of moves.
    pub fn new(data: [M; P]) -> Profile<M, P> {
        Profile::from_per_player(PerPlayer::new(data))
    }

    /// Create a new strategy from a per-player collection of moves.
    pub fn from_per_player(moves: PerPlayer<M, P>) -> Profile<M, P> {
        Profile(moves)
    }

    /// Get a reference to the underlying per-player collection of moves.
    pub fn per_player(&self) -> &PerPlayer<M, P> {
        &self.0
    }

    /// Get a mutable reference to the underlying per-player collection of moves.
    pub fn per_player_mut(&mut self) -> &mut PerPlayer<M, P> {
        &mut self.0
    }
}

impl<M: Move, const P: usize> Profile<M, P> {
    /// Attempt to construct a profile from the given transcript.
    ///
    /// Returns `None` if the transcript does not contain exactly one move per player.
    pub fn from_transcript(transcript: Transcript<M, P>) -> Option<Self> {
        transcript.to_profile()
    }
}

impl<M: Move, const P: usize> Record<M, P> for Profile<M, P> {
    fn plies(&self) -> Plies<M, P> {
        Plies::from_iter(
            P,
            self.as_ref()
                .map_with_index(|p, m| Ply::player(p, m))
                .into_iter(),
        )
    }

    fn summary(&self) -> Summary<P> {
        Summary::simultaneous()
    }

    fn played_moves_by_chance(&self) -> PlayedMoves<M> {
        PlayedMoves::empty()
    }

    fn played_moves_by_player(&self, player: PlayerIndex<P>) -> PlayedMoves<M> {
        PlayedMoves::from_move(self[player])
    }
}

impl<M, const P: usize> Index<PlayerIndex<P>> for Profile<M, P> {
    type Output = M;
    fn index(&self, idx: PlayerIndex<P>) -> &M {
        self.0.for_player(idx)
    }
}

impl<M, const P: usize> IndexMut<PlayerIndex<P>> for Profile<M, P> {
    fn index_mut(&mut self, idx: PlayerIndex<P>) -> &mut M {
        self.0.for_player_mut(idx)
    }
}
