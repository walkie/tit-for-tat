use derive_more::{AsMut, AsRef, Index, IndexMut};
use num::{FromPrimitive, Num};
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Add, Mul, Sub};

use crate::{PerPlayer, PlayerIndex};

/// A trait that collects the trait requirements of payoff utility values.
///
/// A blanket implementation covers all types that meet the requirements, so this trait should not
/// be implemented directly.
pub trait Utility: Copy + Debug + Default + Num + PartialEq + PartialOrd + Sized + 'static {}
impl<T: Copy + Debug + Default + Num + PartialEq + PartialOrd + 'static> Utility for T {}

/// A collection containing the utility values awarded to each player at the end of a game.
///
/// This struct is a wrapper around a [`PerPlayer`] collection. A payoff of type `Payoff<U, P>`
/// awards a (numeric) utility value of type `U` to each player in a game among `P` players.
///
/// # Constructing payoffs
///
/// The simplest way to construct a payoff is to build it directly from an array of utility values.
///
/// ```
/// use tft::Payoff;
///
/// let p = Payoff::from([2, 3, 0, -1]);
/// ```
///
/// The [`Payoff::flat`] function constructs a payoff in which every player receives the same
/// utility (i.e. a "flat" distribution of utilities). The [`Payoff::zeros`] function constructs a
/// flat distribution of zeros. Note that the the size of the payoff will be determined by the
/// ["const generic"](https://blog.rust-lang.org/2021/02/26/const-generics-mvp-beta.html)
/// argument `P`, which can often be inferred from the context in which the payoff is
/// used.
///
/// ```
/// use tft::Payoff;
///
/// assert_eq!(Payoff::zeros(), Payoff::from([0, 0, 0]));
/// assert_eq!(Payoff::flat(5), Payoff::from([5, 5, 5, 5, 5]));
/// ```
///
/// The utility value of a single player can be set by the [`Payoff::except`] method, which is
/// designed to be chained with payoff constructors, such as [`Payoff::flat`].
///
/// ```
/// use tft::{for4, for6, Payoff};
///
/// assert_eq!(Payoff::flat(-2).except(for4::P1, 5), Payoff::from([-2, 5, -2, -2]));
/// assert_eq!(
///     Payoff::zeros().except(for6::P0, 1).except(for6::P4, 3),
///     Payoff::from([1, 0, 0, 0, 3, 0])
/// );
/// ```
///
/// The functions [`Payoff::zero_sum_winner`] and [`Payoff::zero_sum_loser`] construct
/// [zero-sum](https://en.wikipedia.org/wiki/Zero-sum_game) payoffs in which a single player wins
/// or loses (and all other players draw), respectively.
///
/// ```
/// use tft::{for3, for5, Payoff};
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
/// use tft::{for3, for5, Payoff};
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
///   syntax `p[i]` provided via the [`Index`](std::ops::Index) and
///   [`IndexMut`](std::ops::IndexMut) traits.
///
/// For more information, see the documentation for the [`PerPlayer`] type.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, AsMut, AsRef, Index, IndexMut)]
pub struct Payoff<U, const P: usize> {
    utilities: PerPlayer<U, P>,
}

impl<U: Utility, const P: usize> Payoff<U, P> {
    /// Construct a new payoff from a `PerPlayer` collection of utilities.
    ///
    /// Use [`Payoff::from`] to construct a payoff from a simple array of utilities.
    ///
    /// # Example
    /// ```
    /// use tft::{Payoff, PerPlayer};
    ///
    /// assert_eq!(Payoff::new(PerPlayer::new([2, 0, -2])), Payoff::from([2, 0, -2]));
    /// ```
    pub fn new(utilities: PerPlayer<U, P>) -> Self {
        Payoff { utilities }
    }

    /// Construct a payoff where every player's utility is identical.
    ///
    /// Note that the size of the payoff is determined by the type parameter `P`, which
    /// can often be inferred by context.
    ///
    /// It is often useful to chain one or more applications of the [`Payoff::except`] method after
    /// constructing a flat payoff to adjust the utility for individual players.
    ///
    /// # Examples
    /// ```
    /// use tft::{for8, Payoff};
    ///
    /// assert_eq!(Payoff::flat(2), Payoff::from([2, 2, 2]));
    /// assert_eq!(
    ///     Payoff::flat(1).except(for8::P2, 5).except(for8::P5, -7),
    ///     Payoff::from([1, 1, 5, 1, 1, -7, 1, 1]),
    /// );
    /// ```
    pub fn flat(utility: U) -> Self {
        Payoff::from([utility; P])
    }

    /// Construct a payoff where every player's utility is zero.
    ///
    /// # Examples
    /// ```
    /// use tft::{for7, Payoff};
    ///
    /// assert_eq!(Payoff::zeros(), Payoff::from([0, 0, 0]));
    ///
    /// assert_eq!(
    ///     Payoff::zeros().except(for7::P0, 3).except(for7::P4, -2),
    ///     Payoff::from([3, 0, 0, 0, -2, 0, 0]),
    /// );
    pub fn zeros() -> Self {
        Payoff::flat(U::zero())
    }

    /// Change the utility corresponding to the given player index.
    ///
    /// This method is designed to be chained with a payoff constructor, such as [`Payoff::from`]
    /// or [`Payoff::flat`].
    ///
    /// # Examples
    /// ```
    /// use tft::{for4, for6, Payoff};
    ///
    /// assert_eq!(Payoff::from([1, 2, 3, 4]).except(for4::P2, -1), Payoff::from([1, 2, -1, 4]));
    /// assert_eq!(
    ///     Payoff::zeros().except(for6::P2, -3).except(for6::P4, 3),
    ///     Payoff::from([0, 0, -3, 0, 3, 0])
    /// );
    /// ```
    pub fn except(mut self, player: PlayerIndex<P>, utility: U) -> Self {
        self.utilities[player] = utility;
        self
    }

    /// Get the number of players in the game, which corresponds to the number of elements in the
    /// payoff.
    ///
    /// # Examples
    /// ```
    /// use tft::Payoff;
    ///
    /// assert_eq!(Payoff::from([2, 0, -2]).num_players(), 3);
    /// assert_eq!(Payoff::from([1, 1, 1, -3, 1]).num_players(), 5);
    ///
    /// ```
    pub fn num_players(&self) -> usize {
        P
    }

    /// Get a reference to the utility for the `i`th player in the game. Returns `None` if the
    /// index is out of range.
    ///
    /// # Examples
    /// ```
    /// use tft::Payoff;
    ///
    /// let p = Payoff::from([1, -2, 3]);
    ///
    /// assert_eq!(p.for_player(0), Some(1));
    /// assert_eq!(p.for_player(1), Some(-2));
    /// assert_eq!(p.for_player(2), Some(3));
    /// assert_eq!(p.for_player(3), None);
    /// ```
    pub fn for_player(&self, i: usize) -> Option<U> {
        self.utilities.for_player(i).copied()
    }

    /// Get a mutable reference to the utility for the `i`th player in the game. Returns `None` if
    /// the index is out of range.
    /// ```
    /// use tft::Payoff;
    ///
    /// let mut p = Payoff::from([1, -2, 3]);
    /// *p.for_player_mut(1).unwrap() = 4;
    ///
    /// assert_eq!(p.for_player(0), Some(1));
    /// assert_eq!(p.for_player(1), Some(4));
    /// assert_eq!(p.for_player(2), Some(3));
    /// assert_eq!(p.for_player(3), None);
    /// ```
    pub fn for_player_mut(&mut self, i: usize) -> Option<&mut U> {
        self.utilities.for_player_mut(i)
    }

    /// Is this a zero-sum payoff? That is, do each of the utility values it contains sum to zero?
    ///
    /// # Examples
    /// ```
    /// use tft::{Payoff, PlayerIndex};
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

    /// The amount that a given payoff represents a
    /// [Pareto improvement](https://en.wikipedia.org/wiki/Pareto_efficiency) over this payoff.
    ///
    /// A payoff is a Pareto improvement over another if the first increases at least one utility
    /// value *without decreasing* any others.
    ///
    /// This function returns the sum of all utility value increases, or `None` if the payoff is
    /// not a Pareto improvement over this payoff.
    ///
    /// # Examples
    /// ```
    /// use tft::Payoff;
    ///
    /// assert_eq!(Payoff::from([2.5, 0.3]).pareto_improvement(Payoff::from([3.0, 1.0])), Some(1.2));
    /// assert_eq!(Payoff::from([2.5, 0.3]).pareto_improvement(Payoff::from([2.5, 0.3])), None);
    /// assert_eq!(Payoff::from([2.5, 0.3]).pareto_improvement(Payoff::from([3.0, 0.0])), None);
    ///
    /// assert_eq!(
    ///   Payoff::from([-3, 2, -5, 4]).pareto_improvement(Payoff::from([-3, 4, -4, 4])),
    ///   Some(3),
    /// );
    /// assert_eq!(
    ///   Payoff::from([-3, 2, -5, 4]).pareto_improvement(Payoff::from([-3, 100, 0, 3])),
    ///   None,
    /// );
    /// ```
    pub fn pareto_improvement(&self, other: Self) -> Option<U> {
        let mut improvement = U::zero();
        for (v_self, v_other) in self.utilities.into_iter().zip(other.utilities) {
            if v_self.le(&v_other) {
                improvement = improvement.add(v_other.sub(v_self));
            } else {
                return None;
            }
        }
        if improvement.is_zero() {
            None
        } else {
            Some(improvement)
        }
    }

    /// Map a function over all of the elements in a payoff.
    fn map(self, f: impl Fn(U) -> U) -> Self {
        let mut result = [U::zero(); P];
        for (r, v) in result.iter_mut().zip(self) {
            *r = f(v);
        }
        Payoff::from(result)
    }

    /// Combine two payoffs element-wise using the given function.
    fn zip_with(self, other: Self, combine: impl Fn(U, U) -> U) -> Self {
        let mut result = [U::zero(); P];
        for ((r, v), w) in result.iter_mut().zip(self).zip(other) {
            *r = combine(v, w);
        }
        Payoff::from(result)
    }
}

impl<U: Utility + FromPrimitive, const P: usize> Payoff<U, P> {
    /// Construct a zero-sum payoff in which one player "loses" by receiving a utility of
    /// `1-P` while all other players receive a utility of `1`.
    ///
    /// # Examples
    /// ```
    /// use tft::{for4, for7, Payoff};
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
    pub fn zero_sum_loser(loser: PlayerIndex<P>) -> Self {
        let reward = U::one();
        let penalty = U::one().sub(U::from_usize(P).unwrap());
        Payoff::flat(reward).except(loser, penalty)
    }

    /// Construct a zero-sum payoff in which one player "wins" by receiving a utility of
    /// `P-1` while all other players receive a utility `-1`.
    ///
    /// # Examples
    /// ```
    /// use tft::{for1, for4, for7, Payoff};
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
    pub fn zero_sum_winner(winner: PlayerIndex<P>) -> Self {
        let penalty = U::zero().sub(U::one());
        let reward = U::from_usize(P).unwrap().sub(U::one());
        Payoff::flat(penalty).except(winner, reward)
    }
}

impl<U: Utility, const P: usize> From<[U; P]> for Payoff<U, P> {
    /// Construct a payoff from an array of utility values.
    ///
    /// # Examples
    /// ```
    /// use tft::{Payoff, PerPlayer};
    ///
    /// assert_eq!(
    ///   Payoff::from([1, 2, 3, 4]),
    ///   Payoff::new(PerPlayer::new([1, 2, 3, 4]))
    /// );
    /// ```
    fn from(utilities: [U; P]) -> Self {
        Payoff::new(PerPlayer::new(utilities))
    }
}

impl<U: Utility, const P: usize> Add<U> for Payoff<U, P> {
    type Output = Self;

    /// Add a constant value to each utility in a payoff.
    ///
    /// # Examples
    /// ```
    /// use tft::Payoff;
    ///
    /// assert_eq!(Payoff::from([2, -3, 4]) + 10, Payoff::from([12, 7, 14]));
    /// assert_eq!(Payoff::from([0, 12]) + -6, Payoff::from([-6, 6]));
    /// ```
    fn add(self, constant: U) -> Self {
        self.map(|v| v + constant)
    }
}

impl<U: Utility, const P: usize> Sub<U> for Payoff<U, P> {
    type Output = Self;

    /// Subtract a constant value from each utility in a payoff.
    ///
    /// # Examples
    /// ```
    /// use tft::Payoff;
    ///
    /// assert_eq!(Payoff::from([15, 6, 12]) - 10, Payoff::from([5, -4, 2]));
    /// assert_eq!(Payoff::from([-3, 3]) - -6, Payoff::from([3, 9]));
    /// ```
    fn sub(self, constant: U) -> Self {
        self.map(|v| v - constant)
    }
}

impl<U: Utility, const P: usize> Mul<U> for Payoff<U, P> {
    type Output = Self;

    /// Multiply a constant value to each utility in a payoff.
    ///
    /// # Examples
    /// ```
    /// use tft::Payoff;
    ///
    /// assert_eq!(Payoff::from([3, -4, 5]) * 3, Payoff::from([9, -12, 15]));
    /// assert_eq!(Payoff::from([0, 3]) * -2, Payoff::from([0, -6]));
    /// ```
    fn mul(self, constant: U) -> Self {
        self.map(|v| v * constant)
    }
}

impl<U: Utility, const P: usize> Add<Self> for Payoff<U, P> {
    type Output = Self;

    /// Combine two payoffs by adding the corresponding utilities in each.
    ///
    /// # Examples
    /// ```
    /// use tft::Payoff;
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

impl<U: Utility, const P: usize> Sub<Self> for Payoff<U, P> {
    type Output = Self;

    /// Combine two payoffs by subtracting the corresponding utilities in the second payoff from
    /// those in the first payoff.
    ///
    /// # Examples
    /// ```
    /// use tft::Payoff;
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

impl<U: Utility, const P: usize> Mul<Self> for Payoff<U, P> {
    type Output = Self;

    /// Combine two payoffs by multiplying the corresponding utilities in each.
    ///
    /// # Examples
    /// ```
    /// use tft::Payoff;
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

impl<U, const P: usize> IntoIterator for Payoff<U, P> {
    type Item = <PerPlayer<U, P> as IntoIterator>::Item;
    type IntoIter = <PerPlayer<U, P> as IntoIterator>::IntoIter;
    fn into_iter(self) -> <PerPlayer<U, P> as IntoIterator>::IntoIter {
        self.utilities.into_iter()
    }
}

impl<'a, U, const P: usize> IntoIterator for &'a Payoff<U, P> {
    type Item = <&'a PerPlayer<U, P> as IntoIterator>::Item;
    type IntoIter = <&'a PerPlayer<U, P> as IntoIterator>::IntoIter;
    fn into_iter(self) -> <&'a PerPlayer<U, P> as IntoIterator>::IntoIter {
        (&self.utilities).into_iter()
    }
}

impl<'a, U, const P: usize> IntoIterator for &'a mut Payoff<U, P> {
    type Item = <&'a mut PerPlayer<U, P> as IntoIterator>::Item;
    type IntoIter = <&'a mut PerPlayer<U, P> as IntoIterator>::IntoIter;
    fn into_iter(self) -> <&'a mut PerPlayer<U, P> as IntoIterator>::IntoIter {
        (&mut self.utilities).into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{for1, for2, for3, for4};
    use test_log::test;

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

    #[test]
    fn default_is_zeros() {
        assert_eq!(Payoff::<u8, 5>::default(), Payoff::zeros());
        assert_eq!(Payoff::<f64, 7>::default(), Payoff::zeros());
        assert_eq!(Payoff::<i32, 101>::default(), Payoff::zeros());
    }
}
