use derive_more::{From, Index, IndexMut, Into};
use num::{FromPrimitive, Num};

use crate::seq::player::{PerPlayer, PlayerIdx};

#[derive(Clone, Debug, Eq, PartialEq, From, Into, Index, IndexMut)]
pub struct Payoff<T, const NUM_PLAYERS: usize> {
    values: PerPlayer<T, NUM_PLAYERS>,
}

impl<T, const NUM_PLAYERS: usize> From<[T; NUM_PLAYERS]> for Payoff<T, NUM_PLAYERS> {
    fn from(values: [T; NUM_PLAYERS]) -> Self {
        Payoff::new(PerPlayer::new(values))
    }
}

impl<T, const NUM_PLAYERS: usize> Payoff<T, NUM_PLAYERS> {
    pub fn new(values: PerPlayer<T, NUM_PLAYERS>) -> Self {
        Payoff { values }
    }

    pub fn except(mut self, player: PlayerIdx<NUM_PLAYERS>, score: T) -> Self {
        self.values[player] = score;
        self
    }
}

impl<T: Copy, const NUM_PLAYERS: usize> Payoff<T, NUM_PLAYERS> {
    pub fn flat(score: T) -> Self {
        Payoff::from([score; NUM_PLAYERS])
    }
}

impl<T: Copy + FromPrimitive + Num, const NUM_PLAYERS: usize> Payoff<T, NUM_PLAYERS> {
    pub fn zero_sum_loser(loser: PlayerIdx<NUM_PLAYERS>) -> Self {
        let reward = T::one();
        let penalty = T::one().sub(T::from_usize(NUM_PLAYERS).unwrap());
        Payoff::flat(reward).except(loser, penalty)
    }

    pub fn zero_sum_winner(winner: PlayerIdx<NUM_PLAYERS>) -> Self {
        let penalty = T::zero().sub(T::one());
        let reward = T::from_usize(NUM_PLAYERS).unwrap().sub(T::one());
        Payoff::flat(penalty).except(winner, reward)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::seq::player::{for1, for2, for3, for4};

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
