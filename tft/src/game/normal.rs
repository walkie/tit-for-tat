//! Games represented in normal form. Simultaneous move games with finite move sets.

use itertools::Itertools;
use num::Num;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

use crate::core::{Payoff, PerPlayer, PlayerIndex};
use crate::game::{Game, Finite, FiniteSimultaneous, Profile, Simultaneous};
use crate::solution::Dominated;

/// A finite simultaneous-move game represented in [normal form](https://en.wikipedia.org/wiki/Normal-form_game).
///
/// The normal-form representation is essentially a table of payoffs indexed by each player's move.
///
/// Since the payoff table is represented directly, normal-form games must have a finite move set
/// for each player. For games with non-finite move sets, see
/// [`PayoffFn`](crate::game::Simultaneous).
///
/// # Type variables
/// - `Move` -- The type of moves played during the game.
/// - `Util` -- The type of utility value awarded to each player in a payoff.
/// - `N` -- The number of players that play the game.
///
/// # Other normal-form game representations
///
/// This type is the most general normal-form game representation. There are more specific
/// normal-form representations available that should usually be preferred if your game fits their
/// constraints:
/// - [`Bimatrix`] -- 2-players
/// - [`Matrix`] -- 2-players, [zero-sum](https://en.wikipedia.org/wiki/Zero-sum_game)
/// - [`Symmetric`] -- 2-players, [symmetric](https://en.wikipedia.org/wiki/Symmetric_game)
/// - [`Symmetric3`] -- 3-players, symmetric
/// - [`Symmetric4`] -- 4-players, symmetric
///
/// # Examples
#[derive(Clone, Debug)]
pub struct Normal<Move, Util, const N: usize> {
    moves: PerPlayer<Vec<Move>, N>,
    profiles: Vec<PerPlayer<Move, N>>,
    payoffs: HashMap<Profile<Move, N>, Payoff<Util, N>>,
}

impl<Move, Util, const N: usize> Normal<Move, Util, N>
where
    Move: Copy + Debug + Eq + Hash,
    Util: Copy + Debug + Num + Ord,
{
    /// Construct a normal-form game given the list of moves available to each player and a table
    /// of payoffs.
    ///
    /// The payoff table is given as a vector of payoffs in
    /// [row-major order](https://en.wikipedia.org/wiki/Row-_and_column-major_order).
    ///
    /// # Errors
    /// Logs a warning and returns `None` if the number of provided payoffs is fewer than the
    /// number of unique pure strategy profiles. If too many payoffs are provided, the excess
    /// payoffs will be ignored.
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
}

impl<Move, Util, const N: usize> Game<N> for Normal<Move, Util, N>
where
    Move: Copy + Debug + Eq + Hash,
    Util: Copy + Debug + Num + Ord,
{
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
        self.moves[player].contains(the_move)
    }
}

impl<Move, Util, const N: usize> Finite<N> for Normal<Move, Util, N>
where
    Move: Copy + Debug + Eq + Hash,
    Util: Copy + Debug + Num + Ord,
{
    type MoveIter = std::slice::Iter<Move>;

    fn available_moves_for_player_at_state(
        &self,
        player: PlayerIndex<N>,
        _state: &(),
    ) -> Self::MoveIter {
        self.moves[player].iter()
    }
}

impl<Move, Util, const N: usize> Simultaneous<N> for Normal<Move, Util, N>
where
    Move: Copy + Debug + Eq + Hash,
    Util: Copy + Debug + Num + Ord,
{
    fn payoff(&self, profile: Profile<Move, N>) -> Option<Payoff<Util, N>> {
        self.payoffs.get(&profile)
    }
}

impl<Move, Util, const N: usize> FiniteSimultaneous<N> for Normal<Move, Util, N>
where
    Move: Copy + Debug + Eq + Hash,
    Util: Copy + Debug + Num + Ord,
{
    type ProfileIter = std::slice::Iter<Profile<Move, N>>;

    fn profiles(&self) -> Self::ProfileIter {
        self.profiles
    }

    fn available_moves(&self) -> PerPlayer<Vec<Move>, N> {
        self.moves
    }
}

impl<Move, Util, const N: usize> Normal<Move, Util, N>
where
    Move: Clone + Debug + Eq + Hash,
    Util: Copy + Num,
{
    /// Is this game zero-sum? In a zero-sum game, the utility values of each payoff sum to zero.
    ///
    /// # Examples
    /// ```
    /// use tft::game::Normal;
    ///
    /// let rps = Normal::symmetric_for2(
    ///     vec!["Rock", "Paper", "Scissors"],
    ///     vec![0, -1, 1, 1, 0, -1, -1, 1, 0],
    /// ).unwrap();
    ///
    /// assert!(rps.is_zero_sum());
    ///
    /// let pd = Normal::symmetric_for2(
    ///     vec!["Cooperate", "Defect"],
    ///     vec![2, 0, 3, 1],
    /// ).unwrap();
    ///
    /// assert!(!pd.is_zero_sum());
    /// ```
    pub fn is_zero_sum(&self) -> bool {
        self.payoffs.values().all(|payoff| payoff.is_zero_sum())
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
    ///     vec!['A', 'B', 'C'],
    ///     vec!['D', 'E'],
    ///     vec![-3, -1, 0, 2, 4, 6],
    /// ).unwrap();
    ///
    /// assert!(g.is_zero_sum());
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
    ///     vec!['A', 'B', 'C'],
    ///     vec!['D', 'E'],
    ///     vec![0, 5, 4, 3, 2, 1],
    ///     vec![5, 0, 1, 2, 4, 3],
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
    /// utility values for player `P0`.
    ///
    /// # Examples
    /// ```
    /// use tft::core::{Payoff, PerPlayer};
    /// use tft::game::Normal;
    ///
    /// let pd = Normal::symmetric_for2(
    ///     vec!['C', 'D'],
    ///     vec![2, 0, 3, 1],
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
    /// normal-form game. Constructed from a list of moves available to all players and the utility
    /// values for player `P0`.
    ///
    /// # Examples
    /// ```
    /// use tft::core::{Payoff, PerPlayer};
    /// use tft::game::Normal;
    ///
    /// let pd3 = Normal::symmetric_for3(
    ///     vec!['C', 'D'],
    ///     vec![4, 1, 1, 0, 5, 3, 3, 2],
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
    /// normal-form game. Constructed from a list of moves available to all players and the utility
    /// values for player `P0`.
    ///
    /// # Examples
    /// ```
    /// use tft::core::{Payoff, PerPlayer};
    /// use tft::game::Normal;
    ///
    /// let pd4 = Normal::symmetric_for4(
    ///     vec!['C', 'D'],
    ///     vec![6, 2, 2, 1, 2, 1, 1, 0, 7, 5, 5, 4, 5, 4, 4, 3],
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
                vec!['A', 'B'],
                vec!['C', 'D', 'E'],
                vec!['F', 'G', 'H', 'I'],
            ]),
            std::iter::repeat(Payoff::flat(0)).take(24).collect(),
        )
        .unwrap();

        let profile1 = PerPlayer::new(['A', 'C', 'F']);
        let profile2 = PerPlayer::new(['B', 'D', 'I']);
        let profile3 = PerPlayer::new(['A', 'E', 'G']);

        assert_eq!(
            g.adjacent_profiles_for(for3::P0, &profile1),
            vec![PerPlayer::new(['B', 'C', 'F'])],
        );
        assert_eq!(
            g.adjacent_profiles_for(for3::P0, &profile2),
            vec![PerPlayer::new(['A', 'D', 'I'])],
        );
        assert_eq!(
            g.adjacent_profiles_for(for3::P0, &profile3),
            vec![PerPlayer::new(['B', 'E', 'G'])],
        );
        assert_eq!(
            g.adjacent_profiles_for(for3::P1, &profile1),
            vec![
                PerPlayer::new(['A', 'D', 'F']),
                PerPlayer::new(['A', 'E', 'F'])
            ],
        );
        assert_eq!(
            g.adjacent_profiles_for(for3::P1, &profile2),
            vec![
                PerPlayer::new(['B', 'C', 'I']),
                PerPlayer::new(['B', 'E', 'I'])
            ],
        );
        assert_eq!(
            g.adjacent_profiles_for(for3::P1, &profile3),
            vec![
                PerPlayer::new(['A', 'C', 'G']),
                PerPlayer::new(['A', 'D', 'G'])
            ],
        );
        assert_eq!(
            g.adjacent_profiles_for(for3::P2, &profile1),
            vec![
                PerPlayer::new(['A', 'C', 'G']),
                PerPlayer::new(['A', 'C', 'H']),
                PerPlayer::new(['A', 'C', 'I']),
            ],
        );
        assert_eq!(
            g.adjacent_profiles_for(for3::P2, &profile2),
            vec![
                PerPlayer::new(['B', 'D', 'F']),
                PerPlayer::new(['B', 'D', 'G']),
                PerPlayer::new(['B', 'D', 'H']),
            ],
        );
        assert_eq!(
            g.adjacent_profiles_for(for3::P2, &profile3),
            vec![
                PerPlayer::new(['A', 'E', 'F']),
                PerPlayer::new(['A', 'E', 'H']),
                PerPlayer::new(['A', 'E', 'I']),
            ],
        );
    }
}
