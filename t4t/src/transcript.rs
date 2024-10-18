use crate::{Move, PerPlayer, PlayerIndex, Plies, Ply, Profile, Record, Summary};

/// A transcript of the moves played (so far) in a sequential game.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Transcript<M: Move, const P: usize> {
    /// The sequence of played moves.
    plies: im::Vector<Ply<M, P>>,
    /// The number of moves played by each player.
    summary: Summary<P>,
}

impl<M: Move, const P: usize> Default for Transcript<M, P> {
    fn default() -> Self {
        Transcript {
            plies: im::Vector::new(),
            summary: Summary::empty(),
        }
    }
}

impl<M: Move, const P: usize> Record<M, P> for Transcript<M, P> {
    fn plies(&self) -> Plies<M, P> {
        Plies::from_iter(self.plies.len(), self.plies.clone().into_iter())
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

    /// Construct a transcript from a ply iterator.
    pub fn from_plies(plies: Plies<M, P>) -> Self {
        let mut transcript = Transcript::new();
        for ply in plies {
            transcript.add(ply);
        }
        transcript
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
        self.plies.push_back(ply);
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
    pub fn moves_by(&self, player: Option<PlayerIndex<P>>) -> impl Iterator<Item = M> + '_ {
        self.plies
            .iter()
            .filter(move |played| played.player == player)
            .map(|played| played.the_move)
    }

    /// Get all moves played by chance.
    pub fn moves_by_chance(&self) -> impl Iterator<Item = M> + '_ {
        self.moves_by(None)
    }

    /// Get all moves played by a given player.
    pub fn moves_by_player(&self, player: PlayerIndex<P>) -> impl Iterator<Item = M> + '_ {
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
