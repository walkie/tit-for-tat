//! Non-finite, simultaneous move games.

use crate::core::{IsMove, IsUtility, Payoff, PerPlayer, PlayerIndex};

/// A pure strategy profile for a simultaneous game: one move played by each player.
pub type Profile<Move, const N: usize> = PerPlayer<Move, N>;

/// A simultaneous move game.
///
/// This is the most general form of simultaneous move game. It consists of two functions:
/// 1. A predicate that recognizes valid moves for each player.
/// 2. A function that yields the payoff given the moves played by each player.
///
/// This representation is best used for games with non-finite domains of moves. For games with
/// finite domains of moves, see the [`norm`](crate::norm) module for normal-form games.
///
/// # Type variables
///
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
/// use tft::simo::*;
///
/// let valid_move = |p, n: i32| {
///   if p == for2::P0 {
///     n.rem_euclid(2) == 0 // P0 must pick an even number
///   } else {
///     n.rem_euclid(2) == 1 // P1 must pick an odd number
///   }
/// };
/// let payoff = |profile: Profile<i32, 2>| {
///   Payoff::from([profile[for2::P1], profile[for2::P0]])
/// };
/// let pick_em = Simultaneous::new(valid_move, payoff);
///
/// assert_eq!(pick_em.num_players(), 2);
///
/// assert!(pick_em.is_valid_move(for2::P0, 2));
/// assert!(pick_em.is_valid_move(for2::P1, -3));
/// assert!(!pick_em.is_valid_move(for2::P0, 5));
/// assert!(!pick_em.is_valid_move(for2::P1, -4));
///
/// assert!(pick_em.is_valid_profile(PerPlayer::new([-2, 3])));
/// assert!(!pick_em.is_valid_profile(PerPlayer::new([-2, 4])));
/// assert!(!pick_em.is_valid_profile(PerPlayer::new([-3, 5])));
///
/// assert_eq!(pick_em.payoff(PerPlayer::new([-4, 7])), Payoff::from([7, -4]));
/// assert_eq!(pick_em.payoff(PerPlayer::new([0, -1])), Payoff::from([-1, 0]));
/// ```
pub struct Simultaneous<Move, Util, const N: usize> {
    move_fn: Box<dyn Fn(PlayerIndex<N>, Move) -> bool>,
    payoff_fn: Box<dyn Fn(Profile<Move, N>) -> Payoff<Util, N>>,
}

impl<Move: IsMove, Util: IsUtility, const N: usize> Simultaneous<Move, Util, N> {
    /// Construct a new simultaneous move game given (1) a predicate that determines if a move is
    /// valid for a given player, and (2) a function that yields the payoff given a profile
    /// containing a move played by each player.
    ///
    /// If passed an [invalid profile](Simultaneous::is_valid_profile), the payoff function should
    /// return an arbitrary payoff (rather than, say, panic).
    pub fn new(
        move_fn: impl Fn(PlayerIndex<N>, Move) -> bool + 'static,
        payoff_fn: impl Fn(Profile<Move, N>) -> Payoff<Util, N> + 'static,
    ) -> Self {
        Simultaneous {
            move_fn: Box::new(move_fn),
            payoff_fn: Box::new(payoff_fn),
        }
    }

    /// The number of players this game is for.
    pub fn num_players(&self) -> usize {
        N
    }

    /// Is this a valid move for the given player?
    pub fn is_valid_move(&self, player: PlayerIndex<N>, the_move: Move) -> bool {
        (*self.move_fn)(player, the_move)
    }

    /// Is this a valid strategy profile?
    ///
    /// A profile is valid if each move is valid for the corresponding player.
    pub fn is_valid_profile(&self, profile: Profile<Move, N>) -> bool {
        PlayerIndex::all_indexes().all(|pi| self.is_valid_move(pi, profile[pi]))
    }

    /// Get the payoff for the given strategy profile.
    ///
    /// This method should only be called with [valid](Simultaneous::is_valid_profile) profiles.
    /// For invalid profiles, this method will return an arbitrary payoff.
    pub fn payoff(&self, profile: Profile<Move, N>) -> Payoff<Util, N> {
        (*self.payoff_fn)(profile)
    }
}
