//! Games represented in normal form. Simultaneous move games with finite move sets.

use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

use crate::core::{Payoff, PerPlayer, PlayerIndex};
use crate::game::simultaneous::Profile;

/// A simultaneous move game represented in [normal form](https://en.wikipedia.org/wiki/Normal-form_game).
///
/// The normal form representation is essentially a table of payoffs indexed by each player's move.
///
/// Since the payoff table is represented directly, normal-form games must have a finite move set
/// for each player. For games with non-finite move sets, use
/// [`Simultaneous`](crate::game::Simultaneous).
///
/// # Type variables
/// - `Move` -- The type of moves played during the game.
/// - `Util` -- The type of utility value awarded to each player in a payoff.
/// - `N` -- The number of players that play the game.
///
/// # Examples
pub struct Normal<Move, Util, const N: usize> {
    moves: PerPlayer<Vec<Move>, N>,
    profiles: Vec<PerPlayer<Move, N>>,
    payoffs: HashMap<Profile<Move, N>, Payoff<Util, N>>,
}

impl<Move, Util, const N: usize> Normal<Move, Util, N>
where
    Move: Clone + Debug + Eq + Hash,
    Util: Clone,
{
    /// Construct a normal-form game given the list of moves available to each player and a table
    /// of payoffs.
    ///
    /// The payoff table is given as a vector of payoffs in which all payoffs where player `P0`
    /// played a given move are contiguous, all payoffs where `P0` and `P1` played a given pair of
    /// moves are contiguous, and so on. In other words, the payoff table is given in
    /// ["row major" order](https://en.wikipedia.org/wiki/Matrix_representation).
    ///
    /// This operation may fail if the number of provided payoffs is fewer than the number of
    /// unique pure strategy profiles. If too many payoffs are provided, the excess payoffs will be
    /// ignored.
    ///
    /// # Examples
    pub fn new(moves: PerPlayer<Vec<Move>, N>, table: Vec<Payoff<Util, N>>) -> Option<Self> {
        let profiles: Vec<PerPlayer<Move, N>> = moves
            .clone()
            .into_iter()
            .multi_cartesian_product()
            .map(|vec| PerPlayer::new(vec.try_into().unwrap()))
            .collect();

        let mut payoffs = HashMap::with_capacity(profiles.len());
        for (profile, payoff) in profiles.iter().zip(table) {
            payoffs.insert(profile.clone(), payoff);
        }

        if payoffs.len() == profiles.len() {
            Some(Normal {
                moves,
                profiles,
                payoffs,
            })
        } else {
            None
        }
    }

    /// Construct a [bimatrix game](https://en.wikipedia.org/wiki/Bimatrix_game), a two-player
    /// normal-form game. Constructed from the list of moves and a table of utility values for each
    /// player.
    ///
    /// # Examples
    pub fn bimatrix(
        p0_moves: Vec<Move>,
        p1_moves: Vec<Move>,
        p0_utils: Vec<Util>,
        p1_utils: Vec<Util>,
    ) -> Option<Normal<Move, Util, 2>> {
        let moves = PerPlayer::new([p0_moves, p1_moves]);
        let mut payoffs = Vec::with_capacity(p0_utils.len());
        for (u0, u1) in p0_utils.into_iter().zip(p1_utils) {
            payoffs.push(Payoff::from([u0, u1]));
        }
        Normal::new(moves, payoffs)
    }

    /// Get the available moves for the indicated player.
    pub fn available_moves(&self, player: PlayerIndex<N>) -> &[Move] {
        &self.moves[player]
    }

    /// Is this a valid move for the given player?
    pub fn is_valid_move(&self, player: PlayerIndex<N>, the_move: &Move) -> bool {
        self.moves[player].contains(the_move)
    }

    /// Is the given strategy profile valid? A profile is valid if each move is valid for the
    /// corresponding player.
    pub fn is_valid_profile(&self, profile: &Profile<Move, N>) -> bool {
        PlayerIndex::all_indexes().all(|pi| self.is_valid_move(pi, &profile[pi]))
    }

    /// A list of all pure strategy profiles for this game.
    pub fn profiles(&self) -> &[Profile<Move, N>] {
        &self.profiles
    }

    /// Get the payoff for a given strategy profile. May return `None` if the profile contains an
    /// invalid move for some player.
    pub fn payoff(&self, profile: &Profile<Move, N>) -> Option<&Payoff<Util, N>> {
        self.payoffs.get(profile)
    }

    /// The payoff method should yield a payoff for every valid profile. This function checks
    /// whether this property holds for a given profile.
    ///
    /// It is OK if the payoff method returns a (meaningless) payoff for an invalid profile.
    ///
    /// This function is intended for use in tests.
    pub fn law_valid_profile_yields_payoff(&self, profile: &Profile<Move, N>) -> bool {
        if self.is_valid_profile(profile) {
            self.payoff(profile).is_some()
        } else {
            true // ok to return a meaningless payoff for an invalid profile
        }
    }
}

impl<Move, Util> Normal<Move, Util, 2>
where
    Move: Clone + Debug + Eq + Hash,
    Util: Clone,
{
    /// Construct a [symmetric](https://en.wikipedia.org/wiki/Symmetric_game) two-player
    /// normal-form game. Constructed from a list of moves available to both players and the
    /// payoffs for player `P0`.
    ///
    /// # Examples
    /// ```
    /// use tft::core::{Payoff, PerPlayer};
    /// use tft::game::Normal;
    ///
    /// let pd = Normal::symmetric_for2(
    ///     Vec::from(['C', 'D']),
    ///     Vec::from([2, 0, 3, 1]),
    /// ).unwrap();
    ///
    /// assert_eq!(*pd.payoff(&PerPlayer::new(['C', 'C'])).unwrap(), Payoff::from([2, 2]));
    /// assert_eq!(*pd.payoff(&PerPlayer::new(['C', 'D'])).unwrap(), Payoff::from([0, 3]));
    /// assert_eq!(*pd.payoff(&PerPlayer::new(['D', 'C'])).unwrap(), Payoff::from([3, 0]));
    /// assert_eq!(*pd.payoff(&PerPlayer::new(['D', 'D'])).unwrap(), Payoff::from([1, 1]));
    /// ```
    pub fn symmetric_for2(moves: Vec<Move>, utils: Vec<Util>) -> Option<Normal<Move, Util, 2>> {
        let side = moves.len();
        let size = side * side;
        if utils.len() < size {
            return None;
        }

        let mut payoffs = Vec::with_capacity(size);
        for m0 in 0..side {
            for m1 in 0..side {
                let u0 = utils[m0 * side + m1].clone();
                let u1 = utils[m0 + m1 * side].clone();
                payoffs.push(Payoff::from([u0, u1]));
            }
        }
        Normal::new(PerPlayer::new([moves.clone(), moves]), payoffs)
    }
}

impl<Move, Util> Normal<Move, Util, 3>
where
    Move: Clone + Debug + Eq + Hash,
    Util: Clone,
{
    /// Construct a [symmetric](https://en.wikipedia.org/wiki/Symmetric_game) three-player
    /// normal-form game. Constructed from a list of moves available to all players and the payoffs
    /// for player `P0`.
    ///
    /// # Examples
    /// ```
    /// use tft::core::{Payoff, PerPlayer};
    /// use tft::game::Normal;
    ///
    /// let pd3 = Normal::symmetric_for3(
    ///     Vec::from(['C', 'D']),
    ///     Vec::from([4, 1, 1, 0, 5, 3, 3, 2]),
    /// ).unwrap();
    ///
    /// assert_eq!(*pd3.payoff(&PerPlayer::new(['C', 'C', 'C'])).unwrap(), Payoff::from([4, 4, 4]));
    /// assert_eq!(*pd3.payoff(&PerPlayer::new(['C', 'C', 'D'])).unwrap(), Payoff::from([1, 1, 5]));
    /// assert_eq!(*pd3.payoff(&PerPlayer::new(['C', 'D', 'C'])).unwrap(), Payoff::from([1, 5, 1]));
    /// assert_eq!(*pd3.payoff(&PerPlayer::new(['C', 'D', 'D'])).unwrap(), Payoff::from([0, 3, 3]));
    /// assert_eq!(*pd3.payoff(&PerPlayer::new(['D', 'C', 'C'])).unwrap(), Payoff::from([5, 1, 1]));
    /// assert_eq!(*pd3.payoff(&PerPlayer::new(['D', 'C', 'D'])).unwrap(), Payoff::from([3, 0, 3]));
    /// assert_eq!(*pd3.payoff(&PerPlayer::new(['D', 'D', 'C'])).unwrap(), Payoff::from([3, 3, 0]));
    /// assert_eq!(*pd3.payoff(&PerPlayer::new(['D', 'D', 'D'])).unwrap(), Payoff::from([2, 2, 2]));
    /// ```
    pub fn symmetric_for3(moves: Vec<Move>, utils: Vec<Util>) -> Option<Normal<Move, Util, 3>> {
        let side = moves.len();
        let side_pow2 = side.pow(2);
        let size = side.pow(3);
        if utils.len() < size {
            return None;
        }

        let mut payoffs = Vec::with_capacity(size);
        for m0 in 0..side {
            for m1 in 0..side {
                for m2 in 0..side {
                    let u0 = utils[m0 * side_pow2 + m1 * side + m2].clone();
                    let u1 = utils[m0 + m1 * side_pow2 + m2 * side].clone();
                    let u2 = utils[m0 * side + m1 + m2 * side_pow2].clone();
                    payoffs.push(Payoff::from([u0, u1, u2]));
                }
            }
        }
        Normal::new(PerPlayer::new([moves.clone(), moves.clone(), moves]), payoffs)
    }
}

impl<Move, Util> Normal<Move, Util, 4>
where
    Move: Clone + Debug + Eq + Hash,
    Util: Clone,
{
    /// Construct a [symmetric](https://en.wikipedia.org/wiki/Symmetric_game) four-player
    /// normal-form game. Constructed from a list of moves available to all players and the payoffs
    /// for player `P0`.
    ///
    /// # Examples
    /// ```
    /// use tft::core::{Payoff, PerPlayer};
    /// use tft::game::Normal;
    ///
    /// let pd4 = Normal::symmetric_for4(
    ///     Vec::from(['C', 'D']),
    ///     Vec::from([6, 2, 2, 1, 2, 1, 1, 0, 7, 5, 5, 4, 5, 4, 4, 3]),
    /// ).unwrap();
    ///
    /// assert_eq!(*pd4.payoff(&PerPlayer::new(['C', 'C', 'C', 'C'])).unwrap(), Payoff::from([6, 6, 6, 6]));
    /// assert_eq!(*pd4.payoff(&PerPlayer::new(['C', 'C', 'C', 'D'])).unwrap(), Payoff::from([2, 2, 2, 7]));
    /// assert_eq!(*pd4.payoff(&PerPlayer::new(['C', 'C', 'D', 'C'])).unwrap(), Payoff::from([2, 2, 7, 2]));
    /// assert_eq!(*pd4.payoff(&PerPlayer::new(['C', 'C', 'D', 'D'])).unwrap(), Payoff::from([1, 1, 5, 5]));
    /// assert_eq!(*pd4.payoff(&PerPlayer::new(['C', 'D', 'C', 'C'])).unwrap(), Payoff::from([2, 7, 2, 2]));
    /// assert_eq!(*pd4.payoff(&PerPlayer::new(['C', 'D', 'C', 'D'])).unwrap(), Payoff::from([1, 5, 1, 5]));
    /// assert_eq!(*pd4.payoff(&PerPlayer::new(['C', 'D', 'D', 'C'])).unwrap(), Payoff::from([1, 5, 5, 1]));
    /// assert_eq!(*pd4.payoff(&PerPlayer::new(['C', 'D', 'D', 'D'])).unwrap(), Payoff::from([0, 4, 4, 4]));
    /// assert_eq!(*pd4.payoff(&PerPlayer::new(['D', 'C', 'C', 'C'])).unwrap(), Payoff::from([7, 2, 2, 2]));
    /// assert_eq!(*pd4.payoff(&PerPlayer::new(['D', 'C', 'C', 'D'])).unwrap(), Payoff::from([5, 1, 1, 5]));
    /// assert_eq!(*pd4.payoff(&PerPlayer::new(['D', 'C', 'D', 'C'])).unwrap(), Payoff::from([5, 1, 5, 1]));
    /// assert_eq!(*pd4.payoff(&PerPlayer::new(['D', 'C', 'D', 'D'])).unwrap(), Payoff::from([4, 0, 4, 4]));
    /// assert_eq!(*pd4.payoff(&PerPlayer::new(['D', 'D', 'C', 'C'])).unwrap(), Payoff::from([5, 5, 1, 1]));
    /// assert_eq!(*pd4.payoff(&PerPlayer::new(['D', 'D', 'C', 'D'])).unwrap(), Payoff::from([4, 4, 0, 4]));
    /// assert_eq!(*pd4.payoff(&PerPlayer::new(['D', 'D', 'D', 'C'])).unwrap(), Payoff::from([4, 4, 4, 0]));
    /// assert_eq!(*pd4.payoff(&PerPlayer::new(['D', 'D', 'D', 'D'])).unwrap(), Payoff::from([3, 3, 3, 3]));
    /// ```
    pub fn symmetric_for4(moves: Vec<Move>, utils: Vec<Util>) -> Option<Normal<Move, Util, 4>> {
        let side = moves.len();
        let side_pow2 = side.pow(2);
        let side_pow3 = side.pow(3);
        let size = side.pow(4);
        if utils.len() < size {
            return None;
        }

        let mut payoffs = Vec::with_capacity(size);
        for m0 in 0..side {
            for m1 in 0..side {
                for m2 in 0..side {
                    for m3 in 0..side {
                        let u0 = utils[m0 * side_pow3 + m1 * side_pow2 + m2 * side + m3].clone();
                        let u1 = utils[m0 + m1 * side_pow3 + m2 * side_pow2 + m3 * side].clone();
                        let u2 = utils[m0 * side + m1 + m2 * side_pow3 + m3 * side_pow2].clone();
                        let u3 = utils[m0 * side_pow2 + m1 * side + m2 + m3 * side_pow3].clone();
                        payoffs.push(Payoff::from([u0, u1, u2, u3]));
                    }
                }
            }
        }
        Normal::new(PerPlayer::new([moves.clone(), moves.clone(), moves.clone(), moves]), payoffs)
    }
}
