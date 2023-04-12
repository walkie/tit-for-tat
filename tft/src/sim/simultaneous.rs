use crate::moves::IsMove;
use crate::payoff::{IsUtility, Payoff};
use crate::per_player::{PerPlayer, PlayerIndex};
use crate::player::Players;

use crate::sim::outcome::Outcome;
use crate::sim::profile::Profile;

/// A [simultaneous game](https://en.wikipedia.org/wiki/Simultaneous_game) in which each player
/// plays a single move without knowledge of the other players' moves.
///
/// This is the most general form of simultaneous move game. It is defined by two functions:
/// 1. A predicate that recognizes valid moves for each player.
/// 2. A function that yields the payoff given the moves played by each player.
///
/// This representation is best used for games with non-finite domains of moves. For games with
/// finite domains of moves, use [`Normal`](crate::sim::Normal).
///
/// # Type variables
///
/// - `Move` -- The type of moves played during the game.
/// - `Util` -- The type of utility value awarded to each player in a payoff.
/// - `N` -- The number of players that play the game.
///
/// # Example
///
/// A simple game where two players pick each other's score. Player `P0` must pick an even score
/// for player `P1`, while `P1` must pick an odd score for `P0`.
///
/// ```
/// use tft::simultaneous::*;
///
/// let valid_move = |p, n: i32| {
///     if p == for2::P0 {
///         n.rem_euclid(2) == 0 // P0 must pick an even number
///     } else {
///         n.rem_euclid(2) == 1 // P1 must pick an odd number
///     }
/// };
/// let payoff = |profile: Profile<i32, 2>| {
///     Payoff::from([profile[for2::P1], profile[for2::P0]])
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
    payoff_fn: Box<dyn Fn(Profile<Move, N>) -> Payoff<Util, N>>,
}

/// Game execution failed because a player played an invalid move.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct InvalidMove<Move, const N: usize>(PlayerIndex<N>, Move);

impl<Move: IsMove, Util: IsUtility, const N: usize> Simultaneous<Move, Util, N> {
    /// Construct a new simultaneous move game given (1) a predicate that determines if a move is
    /// valid for a given player, and (2) a function that yields the payoff given a profile
    /// containing a valid move played by each player.
    pub fn from_payoff_fn<MoveFn, PayoffFn>(move_fn: MoveFn, payoff_fn: PayoffFn) -> Self
    where
        MoveFn: Fn(PlayerIndex<N>, Move) -> bool + 'static,
        PayoffFn: Fn(Profile<Move, N>) -> Payoff<Util, N> + 'static,
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
        Self::from_payoff_fn(move_fn, payoff_fn)
    }

    /// The number of players this game is for.
    pub fn num_players(&self) -> usize {
        N
    }

    /// Get the payoff for the given strategy profile.
    ///
    /// This method may return an arbitrary payoff if given an
    /// [invalid profile](Simultaneous::is_valid_profile).
    pub fn payoff(&self, profile: Profile<Move, N>) -> Payoff<Util, N> {
        (*self.payoff_fn)(profile)
    }

    /// Is this a valid move for the given player?
    pub fn is_valid_move_for_player(&self, player: PlayerIndex<N>, the_move: Move) -> bool {
        (*self.move_fn)(player, the_move)
    }

    /// Is this a valid strategy profile? A profile is valid if each move is valid for the
    /// corresponding player.
    pub fn is_valid_profile(&self, profile: Profile<Move, N>) -> bool {
        PlayerIndex::all_indexes().all(|pi| self.is_valid_move_for_player(pi, profile[pi]))
    }

    /// Play this game with the given players.
    pub fn play(
        &self,
        players: &Players<Move, (), N>
    ) -> Result<Outcome<Move, Util, N>, InvalidMove<Move, N>> {
        let profile = PerPlayer::generate(|i| players[i].next_move(&()));
        for i in PlayerIndex::all_indexes() {
            if !self.is_valid_move_for_player(i, profile[i]) {
                return Err(InvalidMove(i, profile[i]));
            }
        }
        Ok(Outcome::new(profile, self.payoff(profile)))
    }
}
