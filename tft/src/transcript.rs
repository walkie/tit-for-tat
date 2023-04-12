use crate::moves::IsMove;
use crate::per_player::{PerPlayer, PlayerIndex};
use crate::profile::Profile;

/// A move played during a game.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct PlayedMove<Move, const N: usize> {
    /// The player that played the move, or `None` if it was a move of chance.
    pub player: Option<PlayerIndex<N>>,
    /// The move that was played.
    pub the_move: Move,
}

/// A transcript of the moves played (so far) in a sequential game.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Transcript<Move, const N: usize> {
    /// The sequence of played moves.
    moves: Vec<PlayedMove<Move, N>>,
}

impl<Move, const N: usize> PlayedMove<Move, N> {
    /// Construct a new played move.
    pub fn new(player: Option<PlayerIndex<N>>, the_move: Move) -> Self {
        PlayedMove { player, the_move }
    }

    /// Construct a move played by the given player.
    pub fn player(player: PlayerIndex<N>, the_move: Move) -> Self {
        PlayedMove::new(Some(player), the_move)
    }

    /// Construct a move played by chance.
    pub fn chance(the_move: Move) -> Self {
        PlayedMove::new(None, the_move)
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

impl<Move, const N: usize> Default for Transcript<Move, N> {
    fn default() -> Self {
        Transcript { moves: Vec::new() }
    }
}

impl<Move: IsMove, const N: usize> Transcript<Move, N> {
    /// Construct a new, empty transcript.
    pub fn new() -> Self {
        Transcript::default()
    }

    /// Construct a transcript from a vector of played moves.
    pub fn from_played_moves(moves: Vec<PlayedMove<Move, N>>) -> Self {
        Transcript { moves }
    }

    /// Construct a transcript from the given profile.
    pub fn from_profile(profile: Profile<Move, N>) -> Transcript<Move, N> {
        profile.to_transcript()
    }

    /// Convert this transcript to a profile, if possible.
    ///
    /// Returns `None` if the transcript does not contain exactly one move per player.
    pub fn to_profile(&self) -> Option<Profile<Move, N>> {
        if self.moves.len() == N {
            PerPlayer::generate(|player| self.first_move_by_player(player)).all_some()
        } else {
            None
        }
    }

    /// Add a played move to the transcript.
    pub fn add(&mut self, player: Option<PlayerIndex<N>>, the_move: Move) {
        self.moves.push(PlayedMove::new(player, the_move))
    }

    /// Add a move played by a player (not chance) to the transcript.
    pub fn add_by_player(&mut self, player: PlayerIndex<N>, the_move: Move) {
        self.add(Some(player), the_move)
    }

    /// Add a move played by chance to the transcript.
    pub fn add_by_chance(&mut self, the_move: Move) {
        self.add(None, the_move)
    }

    /// Get all moves played by a given player (`Some`) or by chance (`None`).
    pub fn moves_by(&self, player: Option<PlayerIndex<N>>) -> Vec<Move> {
        self.moves
            .iter()
            .filter(|played| played.player == player)
            .map(|played| played.the_move)
            .collect()
    }

    /// Get all moves played by chance.
    pub fn moves_by_chance(&self) -> Vec<Move> {
        self.moves_by(None)
    }

    /// Get all moves played by a given player.
    pub fn moves_by_player(&self, player: PlayerIndex<N>) -> Vec<Move> {
        self.moves_by(Some(player))
    }

    /// Get the first move played by a given player (`Some`) or by chance (`None`).
    ///
    /// Returns `None` if the given player or chance has not played any moves.
    pub fn first_move_by(&self, player: Option<PlayerIndex<N>>) -> Option<Move> {
        self.moves
            .iter()
            .find(|played| played.player == player)
            .map(|played| played.the_move)
    }

    /// Get the first move played by chance.
    ///
    /// Returns `None` if chance has not played any moves.
    pub fn first_move_by_chance(&self) -> Option<Move> {
        self.first_move_by(None)
    }

    /// Get the first move played by a given player.
    ///
    /// Returns `None` if the given player has not played any moves.
    pub fn first_move_by_player(&self, player: PlayerIndex<N>) -> Option<Move> {
        self.first_move_by(Some(player))
    }

    /// Get the last move played by a given player (`Some`) or by chance (`None`).
    ///
    /// Returns `None` if the given player or chance has not played any moves.
    pub fn last_move_by(&self, player: Option<PlayerIndex<N>>) -> Option<Move> {
        self.moves
            .iter()
            .rev()
            .find(|played| played.player == player)
            .map(|played| played.the_move)
    }

    /// Get the last move played by chance.
    ///
    /// Returns `None` if chance has not played any moves.
    pub fn last_move_by_chance(&self) -> Option<Move> {
        self.last_move_by(None)
    }

    /// Get the last move played by a given player.
    ///
    /// Returns `None` if the given player has not played any moves.
    pub fn last_move_by_player(&self, player: PlayerIndex<N>) -> Option<Move> {
        self.last_move_by(Some(player))
    }
}

impl<Move, const N: usize> IntoIterator for Transcript<Move, N> {
    type Item = <Vec<PlayedMove<Move, N>> as IntoIterator>::Item;
    type IntoIter = <Vec<PlayedMove<Move, N>> as IntoIterator>::IntoIter;
    fn into_iter(self) -> <Vec<PlayedMove<Move, N>> as IntoIterator>::IntoIter {
        self.moves.into_iter()
    }
}

impl<'a, Move, const N: usize> IntoIterator for &'a Transcript<Move, N> {
    type Item = <&'a Vec<PlayedMove<Move, N>> as IntoIterator>::Item;
    type IntoIter = <&'a Vec<PlayedMove<Move, N>> as IntoIterator>::IntoIter;
    fn into_iter(self) -> <&'a Vec<PlayedMove<Move, N>> as IntoIterator>::IntoIter {
        self.moves.iter()
    }
}

impl<'a, Move, const N: usize> IntoIterator for &'a mut Transcript<Move, N> {
    type Item = <&'a mut Vec<PlayedMove<Move, N>> as IntoIterator>::Item;
    type IntoIter = <&'a mut Vec<PlayedMove<Move, N>> as IntoIterator>::IntoIter;
    fn into_iter(self) -> <&'a mut Vec<PlayedMove<Move, N>> as IntoIterator>::IntoIter {
        self.moves.iter_mut()
    }
}
