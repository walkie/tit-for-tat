//! This module defines the [`Payoff`] type for representing the outcome of a game.

use derive_more::{AsMut, AsRef, Index, IndexMut};
use num::{FromPrimitive, Num};
use std::ops::{Add, Mul, Sub};

use crate::per_player::{PerPlayer, PlayerIdx};

/// A wrapper around a [`PerPlayer`] collection that contains the (typically numerical) values
/// awarded to each player at the end of a game.
///
/// TODO: Illustrate building payoffs using the builder pattern, zero-sum payoffs, etc.
///
/// The value for a single player can be obtained by indexing into the payoff using either a
/// dynamically checked `usize` index via the [`for_player`](Payoff::for_player) and
/// [`for_player_mut`](Payoff::for_player_mut) methods, or using a statically checked [`PlayerIdx`]
/// index, as described in the documentation for the [`PerPlayer`] type.
#[derive(Clone, Debug, Eq, PartialEq, AsMut, AsRef, Index, IndexMut)]
pub struct Payoff<T, const N: usize> {
    values: PerPlayer<T, N>,
}

impl<T, const N: usize> Payoff<T, N> {
    pub fn new(values: PerPlayer<T, N>) -> Self {
        Payoff { values }
    }

    pub fn except(mut self, player: PlayerIdx<N>, score: T) -> Self {
        self.values[player] = score;
        self
    }

    /// Get the number of players in the game, which corresponds to the number of elements in the
    /// payoff.
    pub fn num_players(&self) -> usize {
        N
    }

    /// Get a reference to the value for the `i`th player in the game. Returns `None` if the index
    /// is out of range.
    pub fn for_player(&self, i: usize) -> Option<&T> {
        self.values.for_player(i)
    }

    /// Get a mutable reference to the element corresponding to the `i`th player in the game.
    /// Returns `None` if the index is out of range.
    pub fn for_player_mut(&mut self, i: usize) -> Option<&mut T> {
        self.values.for_player_mut(i)
    }
}

impl<T: Copy, const N: usize> Payoff<T, N> {
    pub fn flat(score: T) -> Self {
        Payoff::from([score; N])
    }
}

impl<T: Copy + FromPrimitive + Num, const N: usize> Payoff<T, N> {
    pub fn zero_sum_loser(loser: PlayerIdx<N>) -> Self {
        let reward = T::one();
        let penalty = T::one().sub(T::from_usize(N).unwrap());
        Payoff::flat(reward).except(loser, penalty)
    }

    pub fn zero_sum_winner(winner: PlayerIdx<N>) -> Self {
        let penalty = T::zero().sub(T::one());
        let reward = T::from_usize(N).unwrap().sub(T::one());
        Payoff::flat(penalty).except(winner, reward)
    }
}

impl<T: Copy + Num, const N: usize> Payoff<T, N> {
    fn map(self, f: impl Fn(T) -> T) -> Self {
        let mut result = [T::zero(); N];
        for (r, v) in result.iter_mut().zip(self) {
            *r = f(v);
        }
        Payoff::from(result)
    }

    fn zip_with(self, other: Self, combine: impl Fn(T, T) -> T) -> Self {
        let mut result = [T::zero(); N];
        for ((r, a), b) in result.iter_mut().zip(self).zip(other) {
            *r = combine(a, b);
        }
        Payoff::from(result)
    }
}

impl<T, const N: usize> From<[T; N]> for Payoff<T, N> {
    fn from(values: [T; N]) -> Self {
        Payoff::new(PerPlayer::new(values))
    }
}

impl<T: Copy + Num, const N: usize> Add<T> for Payoff<T, N> {
    type Output = Self;

    /// Add a constant to each value in a payoff.
    ///
    /// # Examples
    /// ```
    /// use game_theory::payoff::Payoff;
    ///
    /// assert_eq!(Payoff::from([2, -3, 4]) + 10, Payoff::from([12, 7, 14]));
    /// assert_eq!(Payoff::from([0, 12]) + -6, Payoff::from([-6, 6]));
    /// ```
    fn add(self, constant: T) -> Self {
        self.map(|v| v + constant)
    }
}

impl<T: Copy + Num, const N: usize> Sub<T> for Payoff<T, N> {
    type Output = Self;

    /// Subtract a constant from each value in a payoff.
    ///
    /// # Examples
    /// ```
    /// use game_theory::payoff::Payoff;
    ///
    /// assert_eq!(Payoff::from([15, 6, 12]) - 10, Payoff::from([5, -4, 2]));
    /// assert_eq!(Payoff::from([-3, 3]) - -6, Payoff::from([3, 9]));
    /// ```
    fn sub(self, constant: T) -> Self {
        self.map(|v| v - constant)
    }
}

impl<T: Copy + Num, const N: usize> Mul<T> for Payoff<T, N> {
    type Output = Self;

    /// Multiply a constant to each value in a payoff.
    ///
    /// # Examples
    /// ```
    /// use game_theory::payoff::Payoff;
    ///
    /// assert_eq!(Payoff::from([3, -4, 5]) * 3, Payoff::from([9, -12, 15]));
    /// assert_eq!(Payoff::from([0, 3]) * -2, Payoff::from([0, -6]));
    /// ```
    fn mul(self, constant: T) -> Self {
        self.map(|v| v * constant)
    }
}

impl<T: Copy + Num, const N: usize> Add<Self> for Payoff<T, N> {
    type Output = Self;

    /// Combine two payoffs by component-wise addition.
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

impl<T: Copy + Num, const N: usize> Sub<Self> for Payoff<T, N> {
    type Output = Self;

    /// Combine two payoffs by component-wise subtraction.
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

impl<T: Copy + Num, const N: usize> Mul<Self> for Payoff<T, N> {
    type Output = Self;

    /// Combine two payoffs by component-wise multiplication.
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

impl<T, const N: usize> IntoIterator for Payoff<T, N> {
    type Item = <PerPlayer<T, N> as IntoIterator>::Item;
    type IntoIter = <PerPlayer<T, N> as IntoIterator>::IntoIter;
    fn into_iter(self) -> <PerPlayer<T, N> as IntoIterator>::IntoIter {
        self.values.into_iter()
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a Payoff<T, N> {
    type Item = <&'a PerPlayer<T, N> as IntoIterator>::Item;
    type IntoIter = <&'a PerPlayer<T, N> as IntoIterator>::IntoIter;
    fn into_iter(self) -> <&'a PerPlayer<T, N> as IntoIterator>::IntoIter {
        (&self.values).into_iter()
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a mut Payoff<T, N> {
    type Item = <&'a mut PerPlayer<T, N> as IntoIterator>::Item;
    type IntoIter = <&'a mut PerPlayer<T, N> as IntoIterator>::IntoIter;
    fn into_iter(self) -> <&'a mut PerPlayer<T, N> as IntoIterator>::IntoIter {
        (&mut self.values).into_iter()
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
    }
}
