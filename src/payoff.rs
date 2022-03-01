//! This module defines the [`Payoff`] type for representing the outcome of a game.
//!
//! The `Payoff` type is a wrapper around a [`PerPlayer`] collection containing the (typically
//! numerical) values awarded to each player in a game. The value for a single player can be
//! obtained by indexing into the payoff using the same techniques described in the
//! [`per_player`](crate::per_player) module.

use derive_more::{From, Index, IndexMut, Into};
use num::{FromPrimitive, Num};

use crate::per_player::{PerPlayer, PlayerIdx};

#[derive(Clone, Debug, Eq, PartialEq, From, Into, Index, IndexMut)]
pub struct Payoff<T, const N: usize> {
    values: PerPlayer<T, N>,
}

impl<T, const N: usize> From<[T; N]> for Payoff<T, N> {
    fn from(values: [T; N]) -> Self {
        Payoff::new(PerPlayer::new(values))
    }
}

impl<T, const N: usize> Payoff<T, N> {
    pub fn new(values: PerPlayer<T, N>) -> Self {
        Payoff { values }
    }

    pub fn except(mut self, player: PlayerIdx<N>, score: T) -> Self {
        self.values[player] = score;
        self
    }

    pub fn as_per_player(&self) -> &PerPlayer<T, N> {
        &self.values
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
