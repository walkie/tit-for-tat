//! Simultaneous move games.

mod profile;

pub use crate::game::Game;
pub use profile::*;

use crate::core::{IsMove, IsUtil, Payoff, PerPlayer, PlayerIndex};

/// A [simultaneous game](https://en.wikipedia.org/wiki/Simultaneous_game) in which each player
/// plays a single move without knowledge of the other players' moves.
pub trait IsSimultaneous<const N: usize>: Game<N> {
    /// Get the payoff for the given strategy profile.
    ///
    /// # Errors
    ///
    /// This method *may* return `None` for an [invalid](IsSimultaneous::is_valid_profile) profile.
    /// However, implementors are not required to check the validity of the profile and may also
    /// return an arbitrary payoff.
    fn payoff(&self, profile: Profile<Self::Move, N>) -> Option<Payoff<Self::Util, N>>;

    /// Is this a valid move for the given player?
    fn is_valid_move_for_player(&self, player: PlayerIndex<N>, the_move: Self::Move) -> bool;

    /// Is this a valid strategy profile? A profile is valid if each move is valid for the
    /// corresponding player.
    fn is_valid_profile(&self, profile: Profile<Self::Move, N>) -> bool {
        PlayerIndex::all_indexes().all(|pi| self.is_valid_move_for_player(pi, profile[pi]))
    }
}

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
/// use tft::game::sim::*;
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
/// let pick_em = Simultaneous::from_payoff_fn(valid_move, payoff);
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
/// assert_eq!(pick_em.payoff(PerPlayer::new([-4, 7])), Payoff::from([7, -4]));
/// assert_eq!(pick_em.payoff(PerPlayer::new([0, -1])), Payoff::from([-1, 0]));
/// ```
pub struct Simultaneous<Move, Util, const N: usize> {
    move_fn: Box<dyn Fn(PlayerIndex<N>, Move) -> bool>,
    payoff_fn: Box<dyn Fn(Profile<Move, N>) -> Option<Payoff<Util, N>>>,
}

impl<Move: IsMove, Util: IsUtil, const N: usize> Simultaneous<Move, Util, N> {
    /// Construct a new simultaneous move game given (1) a predicate that determines if a move is
    /// valid for a given player, and (2) a function that yields the payoff given a profile
    /// containing a move played by each player.
    pub fn from_payoff_fn<MoveFn, PayoffFn>(move_fn: MoveFn, payoff_fn: PayoffFn) -> Self
    where
        MoveFn: Fn(PlayerIndex<N>, Move) -> bool + 'static,
        PayoffFn: Fn(Profile<Move, N>) -> Option<Payoff<Util, N>> + 'static,
    {
        Simultaneous {
            move_fn: Box::new(move_fn),
            payoff_fn: Box::new(payoff_fn),
        }
    }

    /// Construct a new simultaneous move game given (1) a predicate that determines if a move is
    /// valid for a given player, and (2) a utility function for each player.
    pub fn from_utility_fns<MoveFn, UtilFn>(move_fn: MoveFn, util_fns: PerPlayer<UtilFn, N>) -> Self
    where
        MoveFn: Fn(PlayerIndex<N>, Move) -> bool + 'static,
        UtilFn: Fn(Move) -> Util + 'static,
    {
        let payoff_fn = move |profile: Profile<Move, N>| {
            Payoff::new(PerPlayer::generate(|player| {
                util_fns[player](profile[player])
            }))
        };
        Simultaneous {
            move_fn: Box::new(move_fn),
            payoff_fn: Box::new(payoff_fn),
        }
    }
}

impl<Move: IsMove, Util: IsUtil, const N: usize> Game<N> for Simultaneous<Move, Util, N> {
    type Move = Move;
    type Util = Util;
}

impl<Move: IsMove, Util: IsUtil, const N: usize> IsSimultaneous<N> for Simultaneous<Move, Util, N> {
    fn payoff(&self, profile: Profile<Move, N>) -> Payoff<Util, N> {
        (*self.payoff_fn)(profile)
    }
    fn is_valid_move_for_player(&self, player: PlayerIndex<N>, the_move: Move) -> bool {
        (*self.move_fn)(player, the_move)
    }
}
