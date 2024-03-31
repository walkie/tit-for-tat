use crate::{PerPlayer, PlayerIndex};
use std::ops::Add;

/// Tracks the number of moves played so far in a game.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Summary<const P: usize> {
    /// The number of moves played by each player.
    players: PerPlayer<usize, P>,
    /// The number of moves played by chance.
    chance: usize,
    /// The total number of moves played.
    total: usize,
}

impl<const P: usize> Default for Summary<P> {
    fn default() -> Self {
        Summary {
            players: PerPlayer::init_with(0),
            chance: 0,
            total: 0,
        }
    }
}

impl<const P: usize> Summary<P> {
    /// Construct a new move summary with the given move counts for each player and chance.
    ///
    /// # Example
    /// ```
    /// use t4t::{PerPlayer, Summary};
    ///
    /// let s = Summary::new(PerPlayer::new([1, 2, 3]), 4);
    ///
    /// assert_eq!(s.number_of_moves_per_player(), PerPlayer::new([1, 2, 3]));
    /// assert_eq!(s.number_of_moves_by_chance(), 4);
    /// assert_eq!(s.total_number_of_moves(), 10);
    pub fn new(players: PerPlayer<usize, P>, chance: usize) -> Self {
        let total = players.iter().sum::<usize>() + chance;
        Summary {
            players,
            chance,
            total,
        }
    }

    /// Construct a new move summary with all counts set to zero.
    ///
    /// # Example
    /// ```
    /// use t4t::{PerPlayer, Summary};
    ///
    /// let s: Summary<2> = Summary::empty();
    ///
    /// assert_eq!(s.number_of_moves_per_player(), PerPlayer::new([0, 0]));
    /// assert_eq!(s.number_of_moves_by_chance(), 0);
    /// assert_eq!(s.total_number_of_moves(), 0);
    pub fn empty() -> Self {
        Summary::default()
    }

    /// Construct a move summary for a completed simultaneous game where each player has played one
    /// move.
    ///
    /// # Example
    /// ```
    /// use t4t::{PerPlayer, Summary};
    ///
    /// let s: Summary<4> = Summary::simultaneous();
    ///
    /// assert_eq!(s.number_of_moves_per_player(), PerPlayer::new([1, 1, 1, 1]));
    /// assert_eq!(s.number_of_moves_by_chance(), 0);
    /// assert_eq!(s.total_number_of_moves(), 4);
    pub fn simultaneous() -> Self {
        Summary {
            players: PerPlayer::init_with(1),
            chance: 0,
            total: P,
        }
    }

    /// The number of moves made by each player.
    pub fn number_of_moves_per_player(&self) -> PerPlayer<usize, P> {
        self.players
    }

    /// The number of moves made by a particular player.
    pub fn number_of_moves_by_player(&self, player: PlayerIndex<P>) -> usize {
        self.players[player]
    }

    /// The number of moves made by chance.
    pub fn number_of_moves_by_chance(&self) -> usize {
        self.chance
    }

    /// The total number of moves.
    pub fn total_number_of_moves(&self) -> usize {
        self.total
    }

    /// Increment the move count for the given player (`Some`) or chance (`None`).
    pub fn increment_moves_by(&mut self, player: Option<PlayerIndex<P>>) {
        match player {
            Some(p) => self.players[p] += 1,
            None => self.chance += 1,
        }
        self.total += 1;
    }

    /// Increment the move count for the given player.
    ///
    /// # Examples
    /// ```
    /// use t4t::{for3, PerPlayer, Summary};
    ///
    /// let mut s = Summary::empty();
    /// s.increment_moves_by_player(for3::P0);
    /// s.increment_moves_by_player(for3::P2);
    /// s.increment_moves_by_player(for3::P0);
    ///
    /// assert_eq!(s.total_number_of_moves(), 3);
    /// assert_eq!(s.number_of_moves_by_player(for3::P0), 2);
    /// assert_eq!(s.number_of_moves_by_player(for3::P1), 0);
    /// assert_eq!(s.number_of_moves_by_player(for3::P2), 1);
    /// ```
    pub fn increment_moves_by_player(&mut self, player: PlayerIndex<P>) {
        self.increment_moves_by(Some(player))
    }

    /// Increment the move count for chance.
    ///
    /// # Examples
    /// ```
    /// use t4t::{for2, PerPlayer, Summary};
    ///
    /// let mut s = Summary::empty();
    /// s.increment_moves_by_chance();
    /// s.increment_moves_by_player(for2::P1);
    /// s.increment_moves_by_chance();
    ///
    /// assert_eq!(s.total_number_of_moves(), 3);
    /// assert_eq!(s.number_of_moves_by_player(for2::P0), 0);
    /// assert_eq!(s.number_of_moves_by_player(for2::P1), 1);
    /// assert_eq!(s.number_of_moves_by_chance(), 2);
    /// ```
    pub fn increment_moves_by_chance(&mut self) {
        self.increment_moves_by(None)
    }
}

impl<const P: usize> Add<Self> for Summary<P> {
    type Output = Self;

    /// Combine two summaries by adding all of the corresponding move counts.
    ///
    /// # Examples
    /// ```
    /// use t4t::{PerPlayer, Summary};
    ///
    /// assert_eq!(
    ///     Summary::new(PerPlayer::new([1, 2, 3]), 4) + Summary::new(PerPlayer::new([0, 10, 20]), 30),
    ///     Summary::new(PerPlayer::new([1, 12, 23]), 34)
    /// );
    /// ```
    fn add(self, other: Self) -> Self {
        Summary {
            players: self.players.map_with_index(|p, n| n + other.players[p]),
            chance: self.chance + other.chance,
            total: self.total + other.total,
        }
    }
}
