//! This module defines the [`Payoff`] type for representing the outcome of a game.

use derive_more::{AsMut, AsRef, Index, IndexMut};
use num::{FromPrimitive, Num};
use std::ops::{Add, Mul, Sub};

use crate::per_player::{PerPlayer, PlayerIndex};

/// A wrapper around a [`PerPlayer`] collection that contains the (typically numerical) utility
/// values awarded to each player at the end of a game. A payoff of type `Payoff<U, N>` represents
/// a utility value of type `U` awarded to each of the `N` players in a game.
///
/// # Constructing payoffs
///
/// The simplest way to construct a payoff is to build it directly from an array of utility values.
///
/// ```
/// use game_theory::payoff::Payoff;
///
/// let p = Payoff::from([2, 3, 0, -1]);
/// ```
///
/// The [`Payoff::flat`] function constructs a payoff in which every player receives the same
/// utility (i.e. a "flat" distribution of utilities). Note that the the size of the payoff will be
/// determined by the ["const generic"](https://blog.rust-lang.org/2021/02/26/const-generics-mvp-beta.html)
/// argument `N`, which can often be inferred from the context in which the payoff is used.
///
/// ```
/// use game_theory::payoff::Payoff;
///
/// assert_eq!(Payoff::flat(0), Payoff::from([0, 0]));
/// assert_eq!(Payoff::flat(5), Payoff::from([5, 5, 5, 5]));
/// ```
///
/// The utility value of a single player can be set by the [`Payoff::except`] method, which is
/// designed to be chained with payoff constructors, like [`Payoff::flat`].
///
/// ```
/// use game_theory::payoff::Payoff;
/// use game_theory::per_player::{for4, for6};
///
/// assert_eq!(Payoff::flat(-2).except(for4::P1, 5), Payoff::from([-2, 5, -2, -2]));
/// assert_eq!(
///     Payoff::flat(0).except(for6::P0, 1).except(for6::P4, 3),
///     Payoff::from([1, 0, 0, 0, 3, 0])
/// );
/// ```
///
/// The functions [`Payoff::zero_sum_winner`] and [`Payoff::zero_sum_loser`] construct
/// [zero-sum](https://en.wikipedia.org/wiki/Zero-sum_game) payoffs in which a single player wins
/// or loses (and all other players draw), respectively.
///
/// ```
/// use game_theory::payoff::Payoff;
/// use game_theory::per_player::{for3, for5};
///
/// assert_eq!(Payoff::zero_sum_winner(for3::P2), Payoff::from([-1, -1, 2]));
/// assert_eq!(Payoff::zero_sum_loser(for5::P1), Payoff::from([1, -4, 1, 1, 1]));
/// ```
///
/// Finally, payoffs can be constructed from other payoffs using basic arithmetic operations. When
/// added together, two payoffs are combined by adding the corresponding utilities for each player,
/// and similarly for subtraction and multiplication. The right argument of such an arithmetic
/// operation may also be a scalar value, in which case that value is added/subtracted/multiplied
/// from each utility in the payoff.
///
/// ```
/// use game_theory::payoff::Payoff;
/// use game_theory::per_player::{for3, for5};
///
/// assert_eq!(
///     Payoff::from([10, 20, 30]) + Payoff::from([5, 6, 7]),
///     Payoff::from([15, 26, 37])
/// );
/// assert_eq!(
///     Payoff::zero_sum_loser(for5::P1) * 100,
///     Payoff::from([100, -400, 100, 100, 100])
/// );
/// ```
///
/// # Indexing into a payoff to get a single player's utility
///
/// The utility for a single player can be obtained by indexing into the payoff in one of two ways:
///
/// - Using a *dynamically checked* index of type `usize` via the [`for_player`](Payoff::for_player)
///   and [`for_player_mut`](Payoff::for_player_mut) methods.
///
/// - Using a *statically checked* index of type [`PlayerIndex`] and the special Rust indexing
///   syntax `p[i]` provided via the [`Index`] and [`IndexMut`] traits.
///
/// For more information, see the documentation for the [`PerPlayer`] type.
#[derive(Clone, Debug, Eq, PartialEq, AsMut, AsRef, Index, IndexMut)]
pub struct Payoff<U, const N: usize> {
    utilities: PerPlayer<U, N>,
}

impl<U, const N: usize> Payoff<U, N> {
    /// Construct a new payoff from a `PerPlayer` collection of utilities.
    ///
    /// Use [`Payoff::from`] to construct a payoff from a simple array of utilities.
    ///
    /// # Example
    /// ```
    /// use game_theory::payoff::Payoff;
    /// use game_theory::per_player::PerPlayer;
    ///
    /// assert_eq!(Payoff::new(PerPlayer::new([2, 0, -2])), Payoff::from([2, 0, -2]));
    /// ```
    pub fn new(utilities: PerPlayer<U, N>) -> Self {
        Payoff { utilities }
    }

    /// Change the utility corresponding to the given player index.
    ///
    /// This method is designed to be chained with a payoff constructor, such as [`Payoff::from`]
    /// or [`Payoff::flat`].
    ///
    /// # Examples
    /// ```
    /// use game_theory::payoff::Payoff;
    /// use game_theory::per_player::{for4, for6};
    ///
    /// assert_eq!(Payoff::from([1, 2, 3, 4]).except(for4::P2, -1), Payoff::from([1, 2, -1, 4]));
    /// assert_eq!(
    ///     Payoff::flat(0).except(for6::P2, -3).except(for6::P4, 3),
    ///     Payoff::from([0, 0, -3, 0, 3, 0])
    /// );
    /// ```
    pub fn except(mut self, player: PlayerIndex<N>, utility: U) -> Self {
        self.utilities[player] = utility;
        self
    }

    /// Get the number of players in the game, which corresponds to the number of elements in the
    /// payoff.
    ///
    /// # Examples
    /// ```
    /// use game_theory::payoff::Payoff;
    ///
    /// assert_eq!(Payoff::from([2, 0, -2]).num_players(), 3);
    /// assert_eq!(Payoff::from([1, 1, 1, -3, 1]).num_players(), 5);
    ///
    /// ```
    pub fn num_players(&self) -> usize {
        N
    }

    /// Get a reference to the utility for the `i`th player in the game. Returns `None` if the
    /// index is out of range.
    ///
    /// # Examples
    /// ```
    /// use game_theory::payoff::Payoff;
    ///
    /// let p = Payoff::from([1, -2, 3]);
    ///
    /// assert_eq!(p.for_player(0), Some(&1));
    /// assert_eq!(p.for_player(1), Some(&-2));
    /// assert_eq!(p.for_player(2), Some(&3));
    /// assert_eq!(p.for_player(3), None);
    /// ```
    pub fn for_player(&self, i: usize) -> Option<&U> {
        self.utilities.for_player(i)
    }

    /// Get a mutable reference to the utility for the `i`th player in the game. Returns `None` if
    /// the index is out of range.
    /// ```
    /// use game_theory::payoff::Payoff;
    ///
    /// let mut p = Payoff::from([1, -2, 3]);
    /// *p.for_player_mut(1).unwrap() = 4;
    ///
    /// assert_eq!(p.for_player(0), Some(&1));
    /// assert_eq!(p.for_player(1), Some(&4));
    /// assert_eq!(p.for_player(2), Some(&3));
    /// assert_eq!(p.for_player(3), None);
    /// ```
    pub fn for_player_mut(&mut self, i: usize) -> Option<&mut U> {
        self.utilities.for_player_mut(i)
    }
}

impl<U: Copy, const N: usize> Payoff<U, N> {
    /// Construct a payoff where every player's utility is identical.
    ///
    /// Note that the size of the payoff is determined by the type parameter `N`, which can often
    /// be inferred by context.
    ///
    /// It is often useful to chain one or more applications of the [`Payoff::except`] method after
    /// constructing a flat payoff to adjust the utility for individual players.
    ///
    /// # Examples
    /// ```
    /// use game_theory::payoff::Payoff;
    /// use game_theory::per_player::for8;
    ///
    /// assert_eq!(Payoff::flat(2), Payoff::from([2, 2, 2]));
    /// assert_eq!(
    ///     Payoff::flat(0).except(for8::P2, 5).except(for8::P5, -7),
    ///     Payoff::from([0, 0, 5, 0, 0, -7, 0, 0])
    /// );
    /// ```
    pub fn flat(utility: U) -> Self {
        Payoff::from([utility; N])
    }
}

impl<U: Copy + Num, const N: usize> Payoff<U, N> {
    /// Is this a zero-sum payoff? That is, do each of the utility values it contains sum to zero?
    ///
    /// # Examples
    /// ```
    /// use game_theory::payoff::Payoff;
    /// use game_theory::per_player::PlayerIndex;
    ///
    /// assert!(Payoff::<i64, 3>::from([-3, 2, 1]).is_zero_sum());
    /// assert!(Payoff::<i64, 6>::from([0, -10, 3, 0, -1, 8]).is_zero_sum());
    ///
    /// assert!(!Payoff::<i64, 3>::from([-3, 3, 1]).is_zero_sum());
    pub fn is_zero_sum(&self) -> bool {
        let mut sum = U::zero();
        for v in &self.utilities {
            sum = sum.add(*v);
        }
        sum == U::zero()
    }

    fn map(self, f: impl Fn(U) -> U) -> Self {
        let mut result = [U::zero(); N];
        for (r, v) in result.iter_mut().zip(self) {
            *r = f(v);
        }
        Payoff::from(result)
    }

    fn zip_with(self, other: Self, combine: impl Fn(U, U) -> U) -> Self {
        let mut result = [U::zero(); N];
        for ((r, a), b) in result.iter_mut().zip(self).zip(other) {
            *r = combine(a, b);
        }
        Payoff::from(result)
    }
}

impl<U: Copy + FromPrimitive + Num, const N: usize> Payoff<U, N> {
    /// Construct a zero-sum payoff in which one player "loses" by receiving a utility of `1-N`,
    /// while all other players receive a utility of `1`.
    ///
    /// # Examples
    /// ```
    /// use game_theory::payoff::Payoff;
    /// use game_theory::per_player::{for4, for7};
    ///
    /// assert_eq!(
    ///     Payoff::zero_sum_loser(for4::P2),
    ///     Payoff::from([1, 1, -3, 1])
    /// );
    /// assert_eq!(
    ///     Payoff::zero_sum_loser(for7::P2),
    ///     Payoff::from([1, 1, -6, 1, 1, 1, 1])
    /// );
    ///
    /// ```
    pub fn zero_sum_loser(loser: PlayerIndex<N>) -> Self {
        let reward = U::one();
        let penalty = U::one().sub(U::from_usize(N).unwrap());
        Payoff::flat(reward).except(loser, penalty)
    }

    /// Construct a zero-sum payoff in which one player "wins" by receiving a utility of `N-1`,
    /// while all other players receive a utility `-1`.
    ///
    /// # Examples
    /// ```
    /// use game_theory::payoff::Payoff;
    /// use game_theory::per_player::{for1, for4, for7};
    ///
    /// assert_eq!(
    ///     Payoff::zero_sum_winner(for4::P3),
    ///     Payoff::from([-1, -1, -1, 3])
    /// );
    /// assert_eq!(
    ///     Payoff::zero_sum_winner(for7::P3),
    ///     Payoff::from([-1, -1, -1, 6, -1, -1, -1])
    /// );
    ///
    /// ```
    pub fn zero_sum_winner(winner: PlayerIndex<N>) -> Self {
        let penalty = U::zero().sub(U::one());
        let reward = U::from_usize(N).unwrap().sub(U::one());
        Payoff::flat(penalty).except(winner, reward)
    }
}


impl<U, const N: usize> From<[U; N]> for Payoff<U, N> {
    fn from(utilities: [U; N]) -> Self {
        Payoff::new(PerPlayer::new(utilities))
    }
}

impl<U: Copy + Num, const N: usize> Add<U> for Payoff<U, N> {
    type Output = Self;

    /// Add a constant value to each utility in a payoff.
    ///
    /// # Examples
    /// ```
    /// use game_theory::payoff::Payoff;
    ///
    /// assert_eq!(Payoff::from([2, -3, 4]) + 10, Payoff::from([12, 7, 14]));
    /// assert_eq!(Payoff::from([0, 12]) + -6, Payoff::from([-6, 6]));
    /// ```
    fn add(self, constant: U) -> Self {
        self.map(|v| v + constant)
    }
}

impl<U: Copy + Num, const N: usize> Sub<U> for Payoff<U, N> {
    type Output = Self;

    /// Subtract a constant value from each utility in a payoff.
    ///
    /// # Examples
    /// ```
    /// use game_theory::payoff::Payoff;
    ///
    /// assert_eq!(Payoff::from([15, 6, 12]) - 10, Payoff::from([5, -4, 2]));
    /// assert_eq!(Payoff::from([-3, 3]) - -6, Payoff::from([3, 9]));
    /// ```
    fn sub(self, constant: U) -> Self {
        self.map(|v| v - constant)
    }
}

impl<U: Copy + Num, const N: usize> Mul<U> for Payoff<U, N> {
    type Output = Self;

    /// Multiply a constant value to each utility in a payoff.
    ///
    /// # Examples
    /// ```
    /// use game_theory::payoff::Payoff;
    ///
    /// assert_eq!(Payoff::from([3, -4, 5]) * 3, Payoff::from([9, -12, 15]));
    /// assert_eq!(Payoff::from([0, 3]) * -2, Payoff::from([0, -6]));
    /// ```
    fn mul(self, constant: U) -> Self {
        self.map(|v| v * constant)
    }
}

impl<U: Copy + Num, const N: usize> Add<Self> for Payoff<U, N> {
    type Output = Self;

    /// Combine two payoffs by adding the corresponding utilities in each.
    ///
    /// # Examples
    /// ```
    /// use game_theory::payoff::Payoff;
    ///
    /// assert_eq!(
    ///     Payoff::from([10, -20, 30]) + Payoff::from([2, 3, -4]),
    ///     Payoff::from([12, -17, 26])
    /// );
    /// ```
    fn add(self, other_payoff: Self) -> Self {
        self.zip_with(other_payoff, |a, b| a + b)
    }
}

impl<U: Copy + Num, const N: usize> Sub<Self> for Payoff<U, N> {
    type Output = Self;

    /// Combine two payoffs by subtracting the corresponding utilities in the second payoff from
    /// those in the first payoff.
    ///
    /// # Examples
    /// ```
    /// use game_theory::payoff::Payoff;
    ///
    /// assert_eq!(
    ///     Payoff::from([10, -20, 30]) - Payoff::from([2, 3, -4]),
    ///     Payoff::from([8, -23, 34])
    /// );
    /// ```
    fn sub(self, other_payoff: Self) -> Self {
        self.zip_with(other_payoff, |a, b| a - b)
    }
}

impl<U: Copy + Num, const N: usize> Mul<Self> for Payoff<U, N> {
    type Output = Self;

    /// Combine two payoffs by multiplying the corresponding utilities in each.
    ///
    /// # Examples
    /// ```
    /// use game_theory::payoff::Payoff;
    ///
    /// assert_eq!(
    ///     Payoff::from([10, -20, 30]) * Payoff::from([2, 3, -4]),
    ///     Payoff::from([20, -60, -120])
    /// );
    /// ```
    fn mul(self, other_payoff: Self) -> Self {
        self.zip_with(other_payoff, |a, b| a * b)
    }
}

impl<U, const N: usize> IntoIterator for Payoff<U, N> {
    type Item = <PerPlayer<U, N> as IntoIterator>::Item;
    type IntoIter = <PerPlayer<U, N> as IntoIterator>::IntoIter;
    fn into_iter(self) -> <PerPlayer<U, N> as IntoIterator>::IntoIter {
        self.utilities.into_iter()
    }
}

impl<'a, U, const N: usize> IntoIterator for &'a Payoff<U, N> {
    type Item = <&'a PerPlayer<U, N> as IntoIterator>::Item;
    type IntoIter = <&'a PerPlayer<U, N> as IntoIterator>::IntoIter;
    fn into_iter(self) -> <&'a PerPlayer<U, N> as IntoIterator>::IntoIter {
        (&self.utilities).into_iter()
    }
}

impl<'a, U, const N: usize> IntoIterator for &'a mut Payoff<U, N> {
    type Item = <&'a mut PerPlayer<U, N> as IntoIterator>::Item;
    type IntoIter = <&'a mut PerPlayer<U, N> as IntoIterator>::IntoIter;
    fn into_iter(self) -> <&'a mut PerPlayer<U, N> as IntoIterator>::IntoIter {
        (&mut self.utilities).into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::per_player::{for1, for2, for3, for4};

    #[test]
    fn zero_sum_loser_correct() {
        assert_eq!(
            Payoff::<i64, 1>::zero_sum_loser(for1::P0),
            Payoff::from([0])
        );
        assert_eq!(
            Payoff::<i64, 2>::zero_sum_loser(for2::P0),
            Payoff::from([-1, 1])
        );
        assert_eq!(
            Payoff::<i64, 2>::zero_sum_loser(for2::P1),
            Payoff::from([1, -1])
        );
        assert_eq!(
            Payoff::<i64, 3>::zero_sum_loser(for3::P0),
            Payoff::from([-2, 1, 1])
        );
        assert_eq!(
            Payoff::<i64, 3>::zero_sum_loser(for3::P1),
            Payoff::from([1, -2, 1])
        );
        assert_eq!(
            Payoff::<i64, 3>::zero_sum_loser(for3::P2),
            Payoff::from([1, 1, -2])
        );
        assert_eq!(
            Payoff::<i64, 4>::zero_sum_loser(for4::P0),
            Payoff::from([-3, 1, 1, 1])
        );
        assert_eq!(
            Payoff::<i64, 4>::zero_sum_loser(for4::P1),
            Payoff::from([1, -3, 1, 1])
        );
        assert_eq!(
            Payoff::<i64, 4>::zero_sum_loser(for4::P2),
            Payoff::from([1, 1, -3, 1])
        );
        assert_eq!(
            Payoff::<i64, 4>::zero_sum_loser(for4::P3),
            Payoff::from([1, 1, 1, -3])
        );
        for i in 0..100 {
            let p = Payoff::<i64, 100>::zero_sum_loser(PlayerIndex::new(i).unwrap()) * (i as i64);
            assert!(p.is_zero_sum());
        }
    }

    #[test]
    fn zero_sum_winner_correct() {
        assert_eq!(
            Payoff::<i64, 1>::zero_sum_winner(for1::P0),
            Payoff::from([0])
        );
        assert_eq!(
            Payoff::<i64, 2>::zero_sum_winner(for2::P0),
            Payoff::from([1, -1])
        );
        assert_eq!(
            Payoff::<i64, 2>::zero_sum_winner(for2::P1),
            Payoff::from([-1, 1])
        );
        assert_eq!(
            Payoff::<i64, 3>::zero_sum_winner(for3::P0),
            Payoff::from([2, -1, -1])
        );
        assert_eq!(
            Payoff::<i64, 3>::zero_sum_winner(for3::P1),
            Payoff::from([-1, 2, -1])
        );
        assert_eq!(
            Payoff::<i64, 3>::zero_sum_winner(for3::P2),
            Payoff::from([-1, -1, 2])
        );
        assert_eq!(
            Payoff::<i64, 4>::zero_sum_winner(for4::P0),
            Payoff::from([3, -1, -1, -1])
        );
        assert_eq!(
            Payoff::<i64, 4>::zero_sum_winner(for4::P1),
            Payoff::from([-1, 3, -1, -1])
        );
        assert_eq!(
            Payoff::<i64, 4>::zero_sum_winner(for4::P2),
            Payoff::from([-1, -1, 3, -1])
        );
        assert_eq!(
            Payoff::<i64, 4>::zero_sum_winner(for4::P3),
            Payoff::from([-1, -1, -1, 3])
        );
        for i in 0..100 {
            let p = Payoff::<i64, 100>::zero_sum_winner(PlayerIndex::new(i).unwrap()) * (i as i64);
            assert!(p.is_zero_sum());
        }
    }
}
