use crate::{Move, MoveRecord, PerPlayer, PlayerIndex, Ply, PlyIter, Profile};

/// A transcript of the moves played (so far) in a sequential game.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Transcript<M, const P: usize> {
    /// The sequence of played moves.
    moves: Vec<Ply<M, P>>,
}

impl<M, const P: usize> Default for Transcript<M, P> {
    fn default() -> Self {
        Transcript { moves: Vec::new() }
    }
}

impl<M: Move, const P: usize> MoveRecord<M, P> for Transcript<M, P> {
    fn to_iter(&self) -> PlyIter<M, P> {
        PlyIter::new(self.moves.clone().into_iter())
    }

    fn to_transcript(&self) -> Transcript<M, P> {
        self.clone()
    }
}

impl<M: Move, const P: usize> Transcript<M, P> {
    /// Construct a new, empty transcript.
    pub fn new() -> Self {
        Transcript::default()
    }

    /// Construct a transcript from a vector of played moves.
    pub fn from_played_moves(moves: Vec<Ply<M, P>>) -> Self {
        Transcript { moves }
    }

    /// Construct a transcript from the given profile.
    pub fn from_profile(profile: Profile<M, P>) -> Transcript<M, P> {
        profile.to_transcript()
    }

    /// Convert this transcript to a profile, if possible.
    ///
    /// Returns `None` if the transcript does not contain exactly one move per player.
    pub fn to_profile(&self) -> Option<Profile<M, P>> {
        if self.moves.len() == P {
            PerPlayer::generate(|player| self.first_move_by_player(player)).all_some()
        } else {
            None
        }
    }

    /// Add a ply to the transcript.
    pub fn add(&mut self, ply: Ply<M, P>) {
        self.moves.push(ply)
    }

    /// Add a move played by a player (not chance) to the transcript.
    pub fn add_player_move(&mut self, player: PlayerIndex<P>, the_move: M) {
        self.add(Ply::new(Some(player), the_move))
    }

    /// Add a move played by chance to the transcript.
    pub fn add_chance_move(&mut self, the_move: M) {
        self.add(Ply::new(None, the_move))
    }

    /// Get all moves played by a given player (`Some`) or by chance (`None`).
    pub fn moves_by(&self, player: Option<PlayerIndex<P>>) -> Vec<M> {
        self.moves
            .iter()
            .filter(|played| played.player == player)
            .map(|played| played.the_move)
            .collect()
    }

    /// Get all moves played by chance.
    pub fn moves_by_chance(&self) -> Vec<M> {
        self.moves_by(None)
    }

    /// Get all moves played by a given player.
    pub fn moves_by_player(&self, player: PlayerIndex<P>) -> Vec<M> {
        self.moves_by(Some(player))
    }

    /// Get the first move played by a given player (`Some`) or by chance (`None`).
    ///
    /// Returns `None` if the given player or chance has not played any moves.
    pub fn first_move_by(&self, player: Option<PlayerIndex<P>>) -> Option<M> {
        self.moves
            .iter()
            .find(|played| played.player == player)
            .map(|played| played.the_move)
    }

    /// Get the first move played by chance.
    ///
    /// Returns `None` if chance has not played any moves.
    pub fn first_move_by_chance(&self) -> Option<M> {
        self.first_move_by(None)
    }

    /// Get the first move played by a given player.
    ///
    /// Returns `None` if the given player has not played any moves.
    pub fn first_move_by_player(&self, player: PlayerIndex<P>) -> Option<M> {
        self.first_move_by(Some(player))
    }

    /// Get the last move played by a given player (`Some`) or by chance (`None`).
    ///
    /// Returns `None` if the given player or chance has not played any moves.
    pub fn last_move_by(&self, player: Option<PlayerIndex<P>>) -> Option<M> {
        self.moves
            .iter()
            .rev()
            .find(|played| played.player == player)
            .map(|played| played.the_move)
    }

    /// Get the last move played by chance.
    ///
    /// Returns `None` if chance has not played any moves.
    pub fn last_move_by_chance(&self) -> Option<M> {
        self.last_move_by(None)
    }

    /// Get the last move played by a given player.
    ///
    /// Returns `None` if the given player has not played any moves.
    pub fn last_move_by_player(&self, player: PlayerIndex<P>) -> Option<M> {
        self.last_move_by(Some(player))
    }
}

// TODO: Reuse PlyIter here?
impl<M, const P: usize> IntoIterator for Transcript<M, P> {
    type Item = <Vec<Ply<M, P>> as IntoIterator>::Item;
    type IntoIter = <Vec<Ply<M, P>> as IntoIterator>::IntoIter;
    fn into_iter(self) -> <Vec<Ply<M, P>> as IntoIterator>::IntoIter {
        self.moves.into_iter()
    }
}

impl<'a, M, const P: usize> IntoIterator for &'a Transcript<M, P> {
    type Item = <&'a Vec<Ply<M, P>> as IntoIterator>::Item;
    type IntoIter = <&'a Vec<Ply<M, P>> as IntoIterator>::IntoIter;
    fn into_iter(self) -> <&'a Vec<Ply<M, P>> as IntoIterator>::IntoIter {
        self.moves.iter()
    }
}

impl<'a, M, const P: usize> IntoIterator for &'a mut Transcript<M, P> {
    type Item = <&'a mut Vec<Ply<M, P>> as IntoIterator>::Item;
    type IntoIter = <&'a mut Vec<Ply<M, P>> as IntoIterator>::IntoIter;
    fn into_iter(self) -> <&'a mut Vec<Ply<M, P>> as IntoIterator>::IntoIter {
        self.moves.iter_mut()
    }
}
