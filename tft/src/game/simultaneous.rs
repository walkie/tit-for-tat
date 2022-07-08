//! Games where players select their moves simultaneously.

use num::Num;
use std::fmt::Debug;
use std::hash::Hash;

use crate::core::{Payoff, PlayerIndex, Profile};
use crate::game::{Game, Simultaneous};

/// A simultaneous move game represented by a payoff function.
///
/// This is the most general form of simultaneous move game. It actually consists of two functions:
/// 1. A predicate that recognizes valid moves for each player.
/// 2. A function that yields the payoff given the moves played by each player.
///
/// This representation is best used for games with non-finite domains of moves. For games with
/// finite domains of moves, see the various types for normal-form games.
///
/// # Type variables
/// - `Move` -- The type of moves played during the game.
/// - `Util` -- The type of utility value awarded to each player in a payoff.
/// - `N` -- The number of players that play the game.
///
/// # Example
///
/// A simple game where two player's pick each other's score. Player `P0` must pick an even score
/// for player `P1`, while `P1` must pick an odd score for `P0`.
///
/// ```
/// use tft::core::*;
/// use tft::game::*;
///
/// let valid_move = |p, n: i32| {
///   if p == for2::P0 {
///     n.rem_euclid(2) == 0 // P0 must pick an even number
///   } else {
///     n.rem_euclid(2) == 1 // P1 must pick an odd number
///   }
/// };
/// let payoff = |profile: Profile<i32, 2>| {
///   Some(Payoff::from([profile[for2::P1], profile[for2::P0]]))
/// };
/// let pick_em = PayoffFn::new(valid_move, payoff);
///
/// assert_eq!(pick_em.num_players(), 2);
///
/// assert!(pick_em.is_valid_move_for_player(for2::P0, 2));
/// assert!(pick_em.is_valid_move_for_player(for2::P1, -3));
/// assert!(!pick_em.is_valid_move_for_player(for2::P0, 5));
/// assert!(!pick_em.is_valid_move_for_player(for2::P1, -4));
///
/// assert!(pick_em.is_valid_profile(PerPlayer::new([-2, 3])));
/// assert!(!pick_em.is_valid_profile(PerPlayer::new([-2, 4])));
/// assert!(!pick_em.is_valid_profile(PerPlayer::new([-3, 5])));
///
/// assert_eq!(pick_em.payoff(PerPlayer::new([-4, 7])), Some(Payoff::from([7, -4])));
/// assert_eq!(pick_em.payoff(PerPlayer::new([0, -1])), Some(Payoff::from([-1, 0])));
/// ```
pub struct PayoffFn<Move, Util, const N: usize> {
    move_fn: Box<dyn Fn(PlayerIndex<N>, Move) -> bool>,
    payoff_fn: Box<dyn Fn(Profile<Move, N>) -> Option<Payoff<Util, N>>>,
}

impl<Move, Util, const N: usize> PayoffFn<Move, Util, N>
where
    Move: Copy + Debug + Eq + Hash,
    Util: Copy + Debug + Num + Ord,
{
    /// Construct a new simultaneous move game from two functions:
    /// - `move_fn` -- Is the given move valid for the given player?
    /// - `payoff_fn` -- Get the payoff given the moves for each player.
    pub fn new(
        move_fn: impl Fn(PlayerIndex<N>, Move) -> bool + 'static,
        payoff_fn: impl Fn(Profile<Move, N>) -> Option<Payoff<Util, N>> + 'static,
    ) -> Self {
        PayoffFn {
            move_fn: Box::new(move_fn),
            payoff_fn: Box::new(payoff_fn),
        }
    }
}

impl<Move, Util, const N: usize> Game<N> for PayoffFn<Move, Util, N>
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
        (*self.move_fn)(player, the_move)
    }
}

impl<Move, Util, const N: usize> Simultaneous<N> for PayoffFn<Move, Util, N>
where
    Move: Copy + Debug + Eq + Hash,
    Util: Copy + Debug + Num + Ord,
{
    fn payoff(&self, profile: Profile<Move, N>) -> Option<Payoff<Util, N>> {
        (*self.payoff_fn)(profile)
    }
}
