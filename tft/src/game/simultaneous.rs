//! Games where players select their moves simultaneously.

use crate::core::{Payoff, PerPlayer, PlayerIndex};

/// A pure strategy profile: one move played by each player.
pub type Profile<Move, const N: usize> = PerPlayer<Move, N>;

/// A simultaneous move game represented by a predicate that recognizes valid moves and an
/// arbitrary payoff function.
///
/// This is the most general form of simultaneous move game. This representation is best used for
/// games with non-finite domains of moves. For games with finite domains of moves, use
/// [`Normal`](crate::game::Normal), which provides much more functionality.
///
/// # Type variables
/// - `Move` -- The type of moves played during the game.
/// - `Util` -- The type of utility value awarded to each player in a payoff.
/// - `N` -- The number of players that play the game.
///
/// # Example
/// ```
/// use tft::core::{Payoff, PerPlayer, for2};
/// use tft::game::{Profile, Simultaneous};
///
/// // A game where two player's pick each other's score.
/// // P0 must pick an even score for P1, while P1 must pick an odd score for P0.
/// let valid_move = |p, n: &i32| {
///   if p == for2::P0 {
///     n.rem_euclid(2) == 0
///   } else {
///     n.rem_euclid(2) == 1
///   }
/// };
/// let payoff = |profile: &Profile<i32, 2>| {
///   Some(Payoff::from([profile[for2::P1], profile[for2::P0]]))
/// };
/// let pick_em = Simultaneous::new(valid_move, payoff);
///
/// assert_eq!(pick_em.num_players(), 2);
///
/// assert!(pick_em.is_valid_move(for2::P0, &2));
/// assert!(pick_em.is_valid_move(for2::P1, &-3));
/// assert!(!pick_em.is_valid_move(for2::P0, &5));
/// assert!(!pick_em.is_valid_move(for2::P1, &-4));
///
/// assert!(pick_em.is_valid_profile(&PerPlayer::new([-2, 3])));
/// assert!(!pick_em.is_valid_profile(&PerPlayer::new([-2, 4])));
/// assert!(!pick_em.is_valid_profile(&PerPlayer::new([-3, 5])));
///
/// assert_eq!(pick_em.payoff(&PerPlayer::new([-4, 7])), Some(Payoff::from([7, -4])));
/// assert_eq!(pick_em.payoff(&PerPlayer::new([0, -1])), Some(Payoff::from([-1, 0])));
/// ```
pub struct Simultaneous<Move, Util, const N: usize> {
    move_fn: Box<dyn Fn(PlayerIndex<N>, &Move) -> bool>,
    payoff_fn: Box<dyn Fn(&Profile<Move, N>) -> Option<Payoff<Util, N>>>,
}

impl<Move, Util, const N: usize> Simultaneous<Move, Util, N> {
    /// Construct a new simultaneous move game from two functions:
    /// - `move_fn` -- Is the given move valid for the given player?
    /// - `payoff_fn` -- Get the payoff for the given strategy profile.
    pub fn new(
        move_fn: impl Fn(PlayerIndex<N>, &Move) -> bool + 'static,
        payoff_fn: impl Fn(&Profile<Move, N>) -> Option<Payoff<Util, N>> + 'static,
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
    pub fn is_valid_move(&self, player: PlayerIndex<N>, the_move: &Move) -> bool {
        (*self.move_fn)(player, the_move)
    }

    /// Is the given strategy profile valid? A profile is valid if each move is valid for the
    /// corresponding player.
    pub fn is_valid_profile(&self, profile: &Profile<Move, N>) -> bool {
        PlayerIndex::all_indexes().all(|pi| self.is_valid_move(pi, &profile[pi]))
    }

    /// Get the payoff for a given strategy profile. May return `None` if the profile contains an
    /// invalid move for some player.
    pub fn payoff(&self, profile: &Profile<Move, N>) -> Option<Payoff<Util, N>> {
        (*self.payoff_fn)(profile)
    }

    /// The payoff method should yield a payoff for every valid profile. This function checks
    /// whether this property holds for a given profile.
    ///
    /// It is OK if the payoff method returns a (meaningless) payoff for an invalid profile.
    ///
    /// This function is intended for use in tests.
    pub fn law_valid_profile_yields_payoff(&self, profile: &Profile<Move, N>) -> bool {
        if self.is_valid_profile(profile) {
            self.payoff(profile).is_some()
        } else {
            true // ok to return a meaningless payoff for an invalid profile
        }
    }
}
