//! Games represented in normal form. Simultaneous move games with finite move sets.

use std::cmp::Ordering;
use std::collections::HashMap;
use std::iter::Iterator;

use crate::prelude::*;

/// A finite simultaneous-move game represented in [normal form](https://en.wikipedia.org/wiki/Normal-form_game).
///
/// In a normal-form game, each player plays a single move from a finite set of available moves,
/// without knowledge of other players' moves, and the payoff is determined by refering to a table
/// of possible outcomes.
///
/// # Other normal-form game representations
///
/// This type is a general-purpose normal-form game representation, where the payoff table is
/// encoded directly. There are several other normal-form representations available that should
/// usually be preferred if your game fits their special capabilities or constraints:
///
/// - `Bimatrix` -- 2-players
/// - `Matrix` -- 2-players, [zero-sum](https://en.wikipedia.org/wiki/Zero-sum_game)
/// - `Symmetric` -- 2-players, [symmetric](https://en.wikipedia.org/wiki/Symmetric_game)
/// - `Symmetric3` -- 3-players, symmetric
/// - `Symmetric4` -- 4-players, symmetric
///
/// # Examples
#[derive(Clone, Debug)]
pub struct Normal<Move, Util, const N: usize> {
    moves: PerPlayer<Vec<Move>, N>,
    payoff_map: HashMap<Profile<Move, N>, Payoff<Util, N>>,
}

impl<Move: IsMove, Util: IsUtility, const N: usize> Normal<Move, Util, N> {
    /// Construct a normal-form game given the moves available to each player and a vector of
    /// payoffs in [row-major order](https://en.wikipedia.org/wiki/Row-_and_column-major_order).
    ///
    /// # Errors
    ///
    /// This constructor expects the length of the payoff vector to match the number of profiles
    /// that can be generated from the available moves.
    ///
    /// - If *too few* payoffs are provided, logs an error and returns `None`.
    /// - If *too many* payoffs are provided, logs a warning and returns a table in which the
    ///   excess payoffs are ignored.
    ///
    /// # Examples
    pub fn new(moves: PerPlayer<Vec<Move>, N>, payoffs: Vec<Payoff<Util, N>>) -> Option<Self> {
        let profiles: Vec<Profile<Move, N>> = ProfileIter::from_move_vecs(moves.clone()).collect();
        let num_profiles = profiles.len();
        let num_payoffs = payoffs.len();
        match num_profiles.cmp(&num_payoffs) {
            Ordering::Greater => {
                log::error!(
                    "Normal::new: not enough payoffs provided; expected {}, got {}",
                    num_profiles,
                    num_payoffs,
                );
                return None;
            }
            Ordering::Less => {
                log::warn!(
                    "Normal::new: too many payoffs provided; expected {}, got {}",
                    num_profiles,
                    num_payoffs,
                );
            }
            Ordering::Equal => {}
        }
        let mut payoff_map = HashMap::with_capacity(profiles.len());
        for (profile, payoff) in profiles.into_iter().zip(payoffs) {
            payoff_map.insert(profile, payoff);
        }
        Some(Normal { moves, payoff_map })
    }
}

impl<Move: IsMove, Util: IsUtility> Normal<Move, Util, 2> {
    /// Construct a matrix game, a two-player zero-sum game where the payoffs are defined by a
    /// single matrix of utility values.
    ///
    /// Constructed from the list of moves for each player and the matrix (in row major order) of
    /// utility values for player `P0`.
    ///
    /// # Examples
    /// ```
    /// use tft::prelude::*;
    /// use tft::game::Normal;
    ///
    /// let g = Normal::matrix(
    ///     vec!['A', 'B', 'C'],
    ///     vec!['D', 'E'],
    ///     vec![-3, -1, 0, 2, 4, 6],
    /// ).unwrap();
    ///
    /// assert!(g.is_zero_sum());
    /// assert_eq!(g.payoff(PerPlayer::new(['A', 'D'])).unwrap(), Payoff::from([-3, 3]));
    /// assert_eq!(g.payoff(PerPlayer::new(['A', 'E'])).unwrap(), Payoff::from([-1, 1]));
    /// assert_eq!(g.payoff(PerPlayer::new(['B', 'D'])).unwrap(), Payoff::from([0, 0]));
    /// assert_eq!(g.payoff(PerPlayer::new(['B', 'E'])).unwrap(), Payoff::from([2, -2]));
    /// assert_eq!(g.payoff(PerPlayer::new(['C', 'D'])).unwrap(), Payoff::from([4, -4]));
    /// assert_eq!(g.payoff(PerPlayer::new(['C', 'E'])).unwrap(), Payoff::from([6, -6]));
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

    /// Construct a [bimatrix game](https://en.wikipedia.org/wiki/Bimatrix_game), a two-player
    /// game where the payoffs are defined by two matrices of utilities, one for each player.
    ///
    /// Constructed from the list of moves and the matrix (in row major order) of utility values
    /// for each player.
    ///
    /// # Examples
    /// ```
    /// use tft::prelude::*;
    /// use tft::game::Normal;
    ///
    /// let g = Normal::bimatrix(
    ///     vec!['A', 'B', 'C'],
    ///     vec!['D', 'E'],
    ///     vec![0, 5, 4, 3, 2, 1],
    ///     vec![5, 0, 1, 2, 4, 3],
    /// ).unwrap();
    ///
    /// assert_eq!(g.payoff(PerPlayer::new(['A', 'D'])).unwrap(), Payoff::from([0, 5]));
    /// assert_eq!(g.payoff(PerPlayer::new(['A', 'E'])).unwrap(), Payoff::from([5, 0]));
    /// assert_eq!(g.payoff(PerPlayer::new(['B', 'D'])).unwrap(), Payoff::from([4, 1]));
    /// assert_eq!(g.payoff(PerPlayer::new(['B', 'E'])).unwrap(), Payoff::from([3, 2]));
    /// assert_eq!(g.payoff(PerPlayer::new(['C', 'D'])).unwrap(), Payoff::from([2, 4]));
    /// assert_eq!(g.payoff(PerPlayer::new(['C', 'E'])).unwrap(), Payoff::from([1, 3]));
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
    /// utility values for player `P0`.
    ///
    /// # Examples
    /// ```
    /// use tft::prelude::*;
    /// use tft::game::Normal;
    ///
    /// let pd = Normal::symmetric_for2(
    ///     vec!['C', 'D'],
    ///     vec![2, 0, 3, 1],
    /// ).unwrap();
    ///
    /// assert_eq!(pd.payoff(PerPlayer::new(['C', 'C'])).unwrap(), Payoff::from([2, 2]));
    /// assert_eq!(pd.payoff(PerPlayer::new(['C', 'D'])).unwrap(), Payoff::from([0, 3]));
    /// assert_eq!(pd.payoff(PerPlayer::new(['D', 'C'])).unwrap(), Payoff::from([3, 0]));
    /// assert_eq!(pd.payoff(PerPlayer::new(['D', 'D'])).unwrap(), Payoff::from([1, 1]));
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
                let u0 = utils[m0 * side + m1];
                let u1 = utils[m0 + m1 * side];
                payoffs.push(Payoff::from([u0, u1]));
            }
        }
        Normal::new(PerPlayer::new([moves.clone(), moves]), payoffs)
    }
}

impl<Move: IsMove, Util: IsUtility> Normal<Move, Util, 3> {
    /// Construct a [symmetric](https://en.wikipedia.org/wiki/Symmetric_game) three-player
    /// normal-form game. Constructed from a list of moves available to all players and the utility
    /// values for player `P0`.
    ///
    /// # Examples
    /// ```
    /// use tft::prelude::*;
    /// use tft::game::Normal;
    ///
    /// let pd3 = Normal::symmetric_for3(
    ///     vec!['C', 'D'],
    ///     vec![4, 1, 1, 0, 5, 3, 3, 2],
    /// ).unwrap();
    ///
    /// assert_eq!(pd3.payoff(PerPlayer::new(['C', 'C', 'C'])).unwrap(), Payoff::from([4, 4, 4]));
    /// assert_eq!(pd3.payoff(PerPlayer::new(['C', 'C', 'D'])).unwrap(), Payoff::from([1, 1, 5]));
    /// assert_eq!(pd3.payoff(PerPlayer::new(['C', 'D', 'C'])).unwrap(), Payoff::from([1, 5, 1]));
    /// assert_eq!(pd3.payoff(PerPlayer::new(['C', 'D', 'D'])).unwrap(), Payoff::from([0, 3, 3]));
    /// assert_eq!(pd3.payoff(PerPlayer::new(['D', 'C', 'C'])).unwrap(), Payoff::from([5, 1, 1]));
    /// assert_eq!(pd3.payoff(PerPlayer::new(['D', 'C', 'D'])).unwrap(), Payoff::from([3, 0, 3]));
    /// assert_eq!(pd3.payoff(PerPlayer::new(['D', 'D', 'C'])).unwrap(), Payoff::from([3, 3, 0]));
    /// assert_eq!(pd3.payoff(PerPlayer::new(['D', 'D', 'D'])).unwrap(), Payoff::from([2, 2, 2]));
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
                    let u0 = utils[m0 * side_pow2 + m1 * side + m2];
                    let u1 = utils[m0 + m1 * side_pow2 + m2 * side];
                    let u2 = utils[m0 * side + m1 + m2 * side_pow2];
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

impl<Move: IsMove, Util: IsUtility> Normal<Move, Util, 4> {
    /// Construct a [symmetric](https://en.wikipedia.org/wiki/Symmetric_game) four-player
    /// normal-form game. Constructed from a list of moves available to all players and the utility
    /// values for player `P0`.
    ///
    /// # Examples
    /// ```
    /// use tft::prelude::*;
    /// use tft::game::Normal;
    ///
    /// let pd4 = Normal::symmetric_for4(
    ///     vec!['C', 'D'],
    ///     vec![6, 2, 2, 1, 2, 1, 1, 0, 7, 5, 5, 4, 5, 4, 4, 3],
    /// ).unwrap();
    ///
    /// assert_eq!(pd4.payoff(PerPlayer::new(['C', 'C', 'C', 'C'])).unwrap(), Payoff::from([6, 6, 6, 6]));
    /// assert_eq!(pd4.payoff(PerPlayer::new(['C', 'C', 'C', 'D'])).unwrap(), Payoff::from([2, 2, 2, 7]));
    /// assert_eq!(pd4.payoff(PerPlayer::new(['C', 'C', 'D', 'C'])).unwrap(), Payoff::from([2, 2, 7, 2]));
    /// assert_eq!(pd4.payoff(PerPlayer::new(['C', 'C', 'D', 'D'])).unwrap(), Payoff::from([1, 1, 5, 5]));
    /// assert_eq!(pd4.payoff(PerPlayer::new(['C', 'D', 'C', 'C'])).unwrap(), Payoff::from([2, 7, 2, 2]));
    /// assert_eq!(pd4.payoff(PerPlayer::new(['C', 'D', 'C', 'D'])).unwrap(), Payoff::from([1, 5, 1, 5]));
    /// assert_eq!(pd4.payoff(PerPlayer::new(['C', 'D', 'D', 'C'])).unwrap(), Payoff::from([1, 5, 5, 1]));
    /// assert_eq!(pd4.payoff(PerPlayer::new(['C', 'D', 'D', 'D'])).unwrap(), Payoff::from([0, 4, 4, 4]));
    /// assert_eq!(pd4.payoff(PerPlayer::new(['D', 'C', 'C', 'C'])).unwrap(), Payoff::from([7, 2, 2, 2]));
    /// assert_eq!(pd4.payoff(PerPlayer::new(['D', 'C', 'C', 'D'])).unwrap(), Payoff::from([5, 1, 1, 5]));
    /// assert_eq!(pd4.payoff(PerPlayer::new(['D', 'C', 'D', 'C'])).unwrap(), Payoff::from([5, 1, 5, 1]));
    /// assert_eq!(pd4.payoff(PerPlayer::new(['D', 'C', 'D', 'D'])).unwrap(), Payoff::from([4, 0, 4, 4]));
    /// assert_eq!(pd4.payoff(PerPlayer::new(['D', 'D', 'C', 'C'])).unwrap(), Payoff::from([5, 5, 1, 1]));
    /// assert_eq!(pd4.payoff(PerPlayer::new(['D', 'D', 'C', 'D'])).unwrap(), Payoff::from([4, 4, 0, 4]));
    /// assert_eq!(pd4.payoff(PerPlayer::new(['D', 'D', 'D', 'C'])).unwrap(), Payoff::from([4, 4, 4, 0]));
    /// assert_eq!(pd4.payoff(PerPlayer::new(['D', 'D', 'D', 'D'])).unwrap(), Payoff::from([3, 3, 3, 3]));
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
                        let u0 = utils[m0 * side_pow3 + m1 * side_pow2 + m2 * side + m3];
                        let u1 = utils[m0 + m1 * side_pow3 + m2 * side_pow2 + m3 * side];
                        let u2 = utils[m0 * side + m1 + m2 * side_pow3 + m3 * side_pow2];
                        let u3 = utils[m0 * side_pow2 + m1 * side + m2 + m3 * side_pow3];
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

impl<Move: IsMove, Util: IsUtility, const N: usize> Game<N> for Normal<Move, Util, N> {
    type Move = Move;
    type Utility = Util;
    type State = ();

    fn initial_state(&self) {}

    fn is_valid_move_for_player_at_state(
        &self,
        player: PlayerIndex<N>,
        _state: &(),
        the_move: Move,
    ) -> bool {
        self.moves[player].contains(&the_move)
    }
}

impl<Move: IsMove, Util: IsUtility, const N: usize> Finite<N> for Normal<Move, Util, N> {
    fn available_moves_for_player_at_state(
        &self,
        player: PlayerIndex<N>,
        _state: &(),
    ) -> MoveIter<'_, Move> {
        MoveIter::new(self.moves[player].clone().into_iter())
    }
}

impl<Move: IsMove, Util: IsUtility, const N: usize> Simultaneous<N> for Normal<Move, Util, N> {
    fn payoff(&self, profile: Profile<Move, N>) -> Option<Payoff<Util, N>> {
        self.payoff_map.get(&profile).copied()
    }
}

impl<Move: IsMove, Util: IsUtility, const N: usize> FiniteSimultaneous<N>
    for Normal<Move, Util, N>
{
}
