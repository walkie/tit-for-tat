use crate::{Move, PerPlayer, PlayerIndex, Plies, Ply, Profile, Record, Summary};

/// A transcript of the moves played (so far) in a sequential game.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Transcript<M, const P: usize> {
    /// The sequence of played moves.
    plies: Vec<Ply<M, P>>,
    /// The number of moves played by each player.
    summary: Summary<P>,
}

impl<M, const P: usize> Default for Transcript<M, P> {
    fn default() -> Self {
        Transcript {
            plies: Vec::new(),
            summary: Summary::empty(),
        }
    }
}

impl<M: Move, const P: usize> Record<M, P> for Transcript<M, P> {
    fn plies(&self) -> Plies<M, P> {
        Plies::from_vec(self.plies.clone())
    }

    fn summary(&self) -> Summary<P> {
        self.summary
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

    /// Construct a transcript from a vector of plies.
    pub fn from_ply_vec(plies: Vec<Ply<M, P>>) -> Self {
        let mut summary = Summary::empty();
        for ply in &plies {
            summary.increment_moves_by(ply.player)
        }
        Transcript { plies, summary }
    }

    /// Construct a transcript from a ply iterator.
    pub fn from_plies(plies: Plies<M, P>) -> Self {
        Self::from_ply_vec(plies.collect())
    }

    /// Construct a transcript from the given profile.
    pub fn from_profile(profile: Profile<M, P>) -> Transcript<M, P> {
        profile.to_transcript()
    }

    /// Convert this transcript to a profile, if possible.
    ///
    /// Returns `None` if the transcript does not contain exactly one move per player.
    pub fn to_profile(&self) -> Option<Profile<M, P>> {
        if self.summary == Summary::simultaneous() {
            PerPlayer::generate(|player| self.first_move_by_player(player))
                .all_some()
                .map(Profile::from_per_player)
        } else {
            None
        }
    }

    /// Add a ply to the transcript.
    pub fn add(&mut self, ply: Ply<M, P>) {
        self.plies.push(ply);
        match ply.player {
            Some(p) => self.summary.increment_moves_by_player(p),
            None => self.summary.increment_moves_by_chance(),
        }
    }

    /// Add a move played by chance to the transcript.
    pub fn add_chance_move(&mut self, the_move: M) {
        self.add(Ply::new(None, the_move));
        self.summary.increment_moves_by_chance()
    }

    /// Add a move played by a player (not chance) to the transcript.
    pub fn add_player_move(&mut self, player: PlayerIndex<P>, the_move: M) {
        self.add(Ply::new(Some(player), the_move));
        self.summary.increment_moves_by_player(player)
    }

    /// Get all moves played by a given player (`Some`) or by chance (`None`).
    pub fn moves_by(&self, player: Option<PlayerIndex<P>>) -> Vec<M> {
        self.plies
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
        self.plies
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
        self.plies
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

impl<M, const P: usize> Transcript<M, P> {
    /// An iterator over references to plies in the transcript.
    pub fn iter(&self) -> <&Transcript<M, P> as IntoIterator>::IntoIter {
        self.plies.iter()
    }

    /// An iterator over mutable references to plies in the transcript.
    pub fn iter_mut(&mut self) -> <&mut Transcript<M, P> as IntoIterator>::IntoIter {
        self.plies.iter_mut()
    }
}

// TODO: Reuse Plies iterator here?
impl<M, const P: usize> IntoIterator for Transcript<M, P> {
    type Item = <Vec<Ply<M, P>> as IntoIterator>::Item;
    type IntoIter = <Vec<Ply<M, P>> as IntoIterator>::IntoIter;
    fn into_iter(self) -> <Vec<Ply<M, P>> as IntoIterator>::IntoIter {
        self.plies.into_iter()
    }
}

impl<'a, M, const P: usize> IntoIterator for &'a Transcript<M, P> {
    type Item = <&'a Vec<Ply<M, P>> as IntoIterator>::Item;
    type IntoIter = <&'a Vec<Ply<M, P>> as IntoIterator>::IntoIter;
    fn into_iter(self) -> <&'a Vec<Ply<M, P>> as IntoIterator>::IntoIter {
        self.plies.iter()
    }
}

impl<'a, M, const P: usize> IntoIterator for &'a mut Transcript<M, P> {
    type Item = <&'a mut Vec<Ply<M, P>> as IntoIterator>::Item;
    type IntoIter = <&'a mut Vec<Ply<M, P>> as IntoIterator>::IntoIter;
    fn into_iter(self) -> <&'a mut Vec<Ply<M, P>> as IntoIterator>::IntoIter {
        self.plies.iter_mut()
    }
}
