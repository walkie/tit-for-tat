//! Games represented in normal form. Simultaneous move games with finite move sets.

use itertools::Itertools;
use num::Num;
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
    pub fn new(moves: PerPlayer<Vec<Move>, N>, table: Vec<Payoff<Util, N>>) -> Option<Self> {
        let profiles: Vec<PerPlayer<Move, N>> = moves
            .clone()
            .into_iter()
            .multi_cartesian_product()
            .map(|vec| PerPlayer::new(vec.try_into().unwrap()))
            .collect();

        if profiles.len() > table.len() {
            log::warn!(
                "Normal::new(): expected a table of {} payoffs, got only {}",
                profiles.len(),
                table.len()
            );
            return None;
        }

        let mut payoffs = HashMap::with_capacity(profiles.len());
        for (profile, payoff) in profiles.iter().zip(table) {
            payoffs.insert(profile.clone(), payoff);
        }
        Some(Normal {
            moves,
            profiles,
            payoffs,
        })
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

    /// Generate a list of all pure strategy profiles that differ from the given profile only in
    /// the move of the given player.
    ///
    /// # Examples
    /// ```
    /// use tft::core::{for2, Payoff, PerPlayer};
    /// use tft::game::Normal;
    ///
    /// let g = Normal::new(
    ///     PerPlayer::new([
    ///         Vec::from(['A', 'B']),
    ///         Vec::from(['C', 'D', 'E']),
    ///     ]),
    ///     std::iter::repeat(Payoff::flat(0)).take(6).collect(),
    /// ).unwrap();
    ///
    /// assert_eq!(
    ///     g.adjacent_profiles_for(for2::P0, &PerPlayer::new(['A', 'D'])),
    ///     Vec::from([PerPlayer::new(['B', 'D'])]),
    /// );
    /// assert_eq!(
    ///     g.adjacent_profiles_for(for2::P1, &PerPlayer::new(['A', 'D'])),
    ///     Vec::from([PerPlayer::new(['A', 'C']), PerPlayer::new(['A', 'E'])]),
    /// );
    /// ```
    pub fn adjacent_profiles_for(
        &self,
        player: PlayerIndex<N>,
        profile: &Profile<Move, N>,
    ) -> Vec<Profile<Move, N>> {
        let player_moves = self.available_moves(player);
        let mut adjacent = Vec::with_capacity(player_moves.len() - 1);
        for m in player_moves {
            if *m != profile[player] {
                let mut new_profile = profile.clone();
                new_profile[player] = m.clone();
                adjacent.push(new_profile);
            }
        }
        adjacent
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

/// Captures a domination relationship among moves. A move `m1` is dominated by another move `m2`
/// for player `p` if, for any possible moves played by other players, changing from `m1` to `m2`
/// increases `p`'s utility.
pub struct Dominated<Move> {
    pub dominated: Move,
    pub dominator: Move,
}

impl<Move, Util, const N: usize> Normal<Move, Util, N>
where
    Move: Clone + Debug + Eq + Hash,
    Util: Copy + Ord,
{
    /// Return a move that unilaterally improves the given player's utility, if such a move exists.
    ///
    /// A unilateral improvement assumes that all other player's moves will be unchanged.
    ///
    /// # Examples
    /// ```
    /// use tft::core::{for2, PerPlayer};
    /// use tft::game::Normal;
    ///
    /// #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
    /// enum RPS { Rock, Paper, Scissors };
    ///
    /// let rps = Normal::symmetric_for2(
    ///     Vec::from([RPS::Rock, RPS::Paper, RPS::Scissors]),
    ///     Vec::from([ 0, -1,  1,
    ///                 1,  0, -1,
    ///                -1,  1,  0]),
    /// ).unwrap();
    ///
    /// let rock_rock = &PerPlayer::new([RPS::Rock, RPS::Rock]);
    /// assert_eq!(rps.unilaterally_improve(for2::P0, &rock_rock), Some(RPS::Paper));
    /// assert_eq!(rps.unilaterally_improve(for2::P1, &rock_rock), Some(RPS::Paper));
    ///
    /// let paper_scissors = &PerPlayer::new([RPS::Paper, RPS::Scissors]);
    /// assert_eq!(rps.unilaterally_improve(for2::P0, &paper_scissors), Some(RPS::Rock));
    /// assert_eq!(rps.unilaterally_improve(for2::P1, &paper_scissors), None);
    ///
    /// let paper_rock = &PerPlayer::new([RPS::Paper, RPS::Rock]);
    /// assert_eq!(rps.unilaterally_improve(for2::P0, &paper_rock), None);
    /// assert_eq!(rps.unilaterally_improve(for2::P1, &paper_rock), Some(RPS::Scissors));
    /// ```
    pub fn unilaterally_improve(
        &self,
        player: PlayerIndex<N>,
        profile: &Profile<Move, N>,
    ) -> Option<Move> {
        let mut best_move = None;
        let mut best_util = match self.payoff(profile) {
            Some(payoff) => payoff[player],
            None => {
                log::warn!(
                    "Normal::unilaterally_improve(): invalid initial profile: {:?}",
                    profile,
                );
                return best_move;
            }
        };
        for adjacent in self.adjacent_profiles_for(player, profile) {
            let util = self.payoff(&adjacent).unwrap()[player];
            if util > best_util {
                best_move = Some(adjacent[player].clone());
                best_util = util;
            }
        }
        best_move
    }

    /// Is the given strategy profile stable? A profile is stable if no player can unilaterally
    /// improve their utility.
    ///
    /// A stable profile is a pure Nash equilibrium of the game.
    ///
    /// # Examples
    /// ```
    /// use tft::core::PerPlayer;
    /// use tft::game::Normal;
    ///
    /// let dilemma = Normal::symmetric_for2(
    ///     Vec::from(['C', 'D']),
    ///     Vec::from([2, 0, 3, 1]),
    /// ).unwrap();
    ///
    /// let hunt = Normal::symmetric_for2(
    ///     Vec::from(['C', 'D']),
    ///     Vec::from([3, 0, 2, 1]),
    /// ).unwrap();
    ///
    /// let cc = PerPlayer::new(['C', 'C']);
    /// let cd = PerPlayer::new(['C', 'D']);
    /// let dc = PerPlayer::new(['D', 'C']);
    /// let dd = PerPlayer::new(['D', 'D']);
    ///
    /// assert!(!dilemma.is_stable(&cc));
    /// assert!(!dilemma.is_stable(&cd));
    /// assert!(!dilemma.is_stable(&dc));
    /// assert!(dilemma.is_stable(&dd));
    ///
    /// assert!(hunt.is_stable(&cc));
    /// assert!(!hunt.is_stable(&cd));
    /// assert!(!hunt.is_stable(&dc));
    /// assert!(hunt.is_stable(&dd));
    /// ```
    pub fn is_stable(&self, profile: &Profile<Move, N>) -> bool {
        PlayerIndex::all_indexes()
            .all(|player| self.unilaterally_improve(player, profile).is_none())
    }

    /// All pure Nash equilibrium solutions.
    ///
    /// # Examples
    /// ```
    /// use tft::core::PerPlayer;
    /// use tft::game::Normal;
    ///
    /// let dilemma = Normal::symmetric_for2(
    ///     Vec::from(['C', 'D']),
    ///     Vec::from([2, 0, 3, 1]),
    /// ).unwrap();
    ///
    /// let hunt = Normal::symmetric_for2(
    ///     Vec::from(['C', 'D']),
    ///     Vec::from([3, 0, 2, 1]),
    /// ).unwrap();
    ///
    /// assert_eq!(
    ///     dilemma.pure_nash_equilibria(),
    ///     Vec::from([PerPlayer::new(['D', 'D'])]),
    /// );
    /// assert_eq!(
    ///     hunt.pure_nash_equilibria(),
    ///     Vec::from([PerPlayer::new(['C', 'C']), PerPlayer::new(['D', 'D'])]),
    /// );
    /// ```
    pub fn pure_nash_equilibria(&self) -> Vec<Profile<Move, N>> {
        let mut nash = Vec::new();
        for profile in self.profiles() {
            if self.is_stable(profile) {
                nash.push(profile.clone());
            }
        }
        nash
    }

    /// Get the dominated moves for the given player. A move `m1` is dominated by another move `m2`
    /// for player `p` if, for any possible moves played by other players, changing from `m1` to
    /// `m2` increases `p`'s utility.
    pub fn dominated_moves_for(&self, player: PlayerIndex<N>) -> Vec<Dominated<Move>> {
        let mut dominated = Vec::new();
        // TODO
        dominated
    }

    /// Get the dominated moves for each player. A move `m1` is dominated by another move `m2`
    /// for player `p` if, for any possible moves played by other players, changing from `m1`
    /// to `m2` increases `p`'s utility.
    pub fn dominated_moves(&self) -> PerPlayer<Vec<Dominated<Move>>, N> {
        PerPlayer::generate(|index| self.dominated_moves_for(index))
    }
}

impl<Move, Util> Normal<Move, Util, 2>
where
    Move: Clone + Debug + Eq + Hash,
    Util: Copy + Num,
{
    /// Construct a matrix game, a two-player zero-sum game where the payoffs are defined by a
    /// single matrix of utility values.
    ///
    /// Constructed from the list of moves for each player and the matrix (in row major order) of
    /// utility values for player `P0`.
    ///
    /// # Examples
    /// ```
    /// use tft::core::{Payoff, PerPlayer};
    /// use tft::game::Normal;
    ///
    /// let g = Normal::matrix(
    ///     Vec::from(['A', 'B', 'C']),
    ///     Vec::from(['D', 'E']),
    ///     Vec::from([-3, -1, 0, 2, 4, 6]),
    /// ).unwrap();
    ///
    /// assert_eq!(*g.payoff(&PerPlayer::new(['A', 'D'])).unwrap(), Payoff::from([-3, 3]));
    /// assert_eq!(*g.payoff(&PerPlayer::new(['A', 'E'])).unwrap(), Payoff::from([-1, 1]));
    /// assert_eq!(*g.payoff(&PerPlayer::new(['B', 'D'])).unwrap(), Payoff::from([0, 0]));
    /// assert_eq!(*g.payoff(&PerPlayer::new(['B', 'E'])).unwrap(), Payoff::from([2, -2]));
    /// assert_eq!(*g.payoff(&PerPlayer::new(['C', 'D'])).unwrap(), Payoff::from([4, -4]));
    /// assert_eq!(*g.payoff(&PerPlayer::new(['C', 'E'])).unwrap(), Payoff::from([6, -6]));
    /// ```
    pub fn matrix(
        p0_moves: Vec<Move>,
        p1_moves: Vec<Move>,
        p0_utils: Vec<Util>,
    ) -> Option<Normal<Move, Util, 2>> {
        let moves = PerPlayer::new([p0_moves, p1_moves]);
        let mut payoffs = Vec::with_capacity(p0_utils.len());
        for u0 in p0_utils.into_iter() {
            payoffs.push(Payoff::from([u0, Util::zero().sub(u0)]));
        }
        Normal::new(moves, payoffs)
    }
}

impl<Move, Util> Normal<Move, Util, 2>
where
    Move: Clone + Debug + Eq + Hash,
    Util: Clone,
{
    /// Construct a [bimatrix game](https://en.wikipedia.org/wiki/Bimatrix_game), a two-player
    /// game where the payoffs are defined by two matrices of utilities, one for each player.
    ///
    /// Constructed from the list of moves and the matrix (in row major order) of utility values
    /// for each player.
    ///
    /// # Examples
    /// ```
    /// use tft::core::{Payoff, PerPlayer};
    /// use tft::game::Normal;
    ///
    /// let g = Normal::bimatrix(
    ///     Vec::from(['A', 'B', 'C']),
    ///     Vec::from(['D', 'E']),
    ///     Vec::from([0, 5, 4, 3, 2, 1]),
    ///     Vec::from([5, 0, 1, 2, 4, 3]),
    /// ).unwrap();
    ///
    /// assert_eq!(*g.payoff(&PerPlayer::new(['A', 'D'])).unwrap(), Payoff::from([0, 5]));
    /// assert_eq!(*g.payoff(&PerPlayer::new(['A', 'E'])).unwrap(), Payoff::from([5, 0]));
    /// assert_eq!(*g.payoff(&PerPlayer::new(['B', 'D'])).unwrap(), Payoff::from([4, 1]));
    /// assert_eq!(*g.payoff(&PerPlayer::new(['B', 'E'])).unwrap(), Payoff::from([3, 2]));
    /// assert_eq!(*g.payoff(&PerPlayer::new(['C', 'D'])).unwrap(), Payoff::from([2, 4]));
    /// assert_eq!(*g.payoff(&PerPlayer::new(['C', 'E'])).unwrap(), Payoff::from([1, 3]));
    /// ```
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
            log::warn!(
                "Normal::symmetric_for2(): expected {} utility values, got only {}",
                size,
                utils.len(),
            );
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
            log::warn!(
                "Normal::symmetric_for3(): expected {} utility values, got only {}",
                size,
                utils.len(),
            );
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
        Normal::new(
            PerPlayer::new([moves.clone(), moves.clone(), moves]),
            payoffs,
        )
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
            log::warn!(
                "Normal::symmetric_for4(): expected {} utility values, got only {}",
                size,
                utils.len(),
            );
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
        Normal::new(
            PerPlayer::new([moves.clone(), moves.clone(), moves.clone(), moves]),
            payoffs,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{for3, Payoff, PerPlayer};
    use test_log::test;

    #[test]
    fn adjacent_profiles_for3_correct() {
        let g = Normal::new(
            PerPlayer::new([
                Vec::from(['A', 'B']),
                Vec::from(['C', 'D', 'E']),
                Vec::from(['F', 'G', 'H', 'I']),
            ]),
            std::iter::repeat(Payoff::flat(0)).take(24).collect(),
        )
        .unwrap();

        let profile1 = PerPlayer::new(['A', 'C', 'F']);
        let profile2 = PerPlayer::new(['B', 'D', 'I']);
        let profile3 = PerPlayer::new(['A', 'E', 'G']);

        assert_eq!(
            g.adjacent_profiles_for(for3::P0, &profile1),
            Vec::from([PerPlayer::new(['B', 'C', 'F'])]),
        );
        assert_eq!(
            g.adjacent_profiles_for(for3::P0, &profile2),
            Vec::from([PerPlayer::new(['A', 'D', 'I'])]),
        );
        assert_eq!(
            g.adjacent_profiles_for(for3::P0, &profile3),
            Vec::from([PerPlayer::new(['B', 'E', 'G'])]),
        );
        assert_eq!(
            g.adjacent_profiles_for(for3::P1, &profile1),
            Vec::from([
                PerPlayer::new(['A', 'D', 'F']),
                PerPlayer::new(['A', 'E', 'F'])
            ]),
        );
        assert_eq!(
            g.adjacent_profiles_for(for3::P1, &profile2),
            Vec::from([
                PerPlayer::new(['B', 'C', 'I']),
                PerPlayer::new(['B', 'E', 'I'])
            ]),
        );
        assert_eq!(
            g.adjacent_profiles_for(for3::P1, &profile3),
            Vec::from([
                PerPlayer::new(['A', 'C', 'G']),
                PerPlayer::new(['A', 'D', 'G'])
            ]),
        );
        assert_eq!(
            g.adjacent_profiles_for(for3::P2, &profile1),
            Vec::from([
                PerPlayer::new(['A', 'C', 'G']),
                PerPlayer::new(['A', 'C', 'H']),
                PerPlayer::new(['A', 'C', 'I']),
            ]),
        );
        assert_eq!(
            g.adjacent_profiles_for(for3::P2, &profile2),
            Vec::from([
                PerPlayer::new(['B', 'D', 'F']),
                PerPlayer::new(['B', 'D', 'G']),
                PerPlayer::new(['B', 'D', 'H']),
            ]),
        );
        assert_eq!(
            g.adjacent_profiles_for(for3::P2, &profile3),
            Vec::from([
                PerPlayer::new(['A', 'E', 'F']),
                PerPlayer::new(['A', 'E', 'H']),
                PerPlayer::new(['A', 'E', 'I']),
            ]),
        );
    }
}
