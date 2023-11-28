use std::rc::Rc;

use crate::{
    ErrorKind, Game, Move, MoveRecord, Payoff, PerPlayer, PlayerIndex, Profile,
    SimultaneousOutcome, Turn, Utility,
};

/// A [simultaneous game](https://en.wikipedia.org/wiki/Simultaneous_game) in which each player
/// plays a single move without knowledge of the other players' moves.
///
/// This is the most general form of simultaneous move game. It is defined by two functions:
/// 1. A predicate that recognizes valid moves for each player.
/// 2. A function that yields the payoff given the moves played by each player.
///
/// This representation is best used for games with non-finite domains of moves. For games with
/// finite domains of moves, use [`Normal`](crate::Normal).
///
/// # Type variables
///
/// - `M` -- The type of moves played during the game.
/// - `U` -- The type of utility value awarded to each player in a payoff.
/// - `P` -- The number of players that play the game.
///
/// # Example
///
/// A simple game where two players pick each other's score. Player `P0` must pick an even score
/// for player `P1`, while `P1` must pick an odd score for `P0`.
///
/// ```
/// use t4t::*;
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
pub struct Simultaneous<M, U, const P: usize> {
    move_fn: Box<dyn Fn(PlayerIndex<P>, M) -> bool>,
    payoff_fn: Rc<dyn Fn(Profile<M, P>) -> Payoff<U, P>>,
}

impl<M: Move, U: Utility, const P: usize> Game<P> for Simultaneous<M, U, P> {
    type Move = M;
    type Utility = U;
    type Outcome = SimultaneousOutcome<M, U, P>;
    type State = ();
    type View = ();

    fn rules(&self) -> Turn<Self, P> {
        let state = Rc::new(());
        Turn::all_players(state.clone(), move |_, profile| {
            for ply in profile.to_iter() {
                let player = ply.player.unwrap();
                if !self.is_valid_move_for_player(player, ply.the_move) {
                    return Err(ErrorKind::InvalidMove(player, ply.the_move));
                }
            }
            Ok(Turn::end(
                state,
                SimultaneousOutcome::new(profile, self.payoff(profile)),
            ))
        })
    }

    fn state_view(&self, _state: &(), _player: PlayerIndex<P>) {}

    fn is_valid_move(&self, _state: &(), player: PlayerIndex<P>, the_move: M) -> bool {
        self.is_valid_move_for_player(player, the_move)
    }
}

impl<M: Move, U: Utility, const P: usize> Simultaneous<M, U, P> {
    /// Construct a new simultaneous move game given (1) a predicate that determines if a move is
    /// valid for a given player, and (2) a function that yields the payoff given a profile
    /// containing a valid move played by each player.
    pub fn from_payoff_fn<MoveFn, PayoffFn>(move_fn: MoveFn, payoff_fn: PayoffFn) -> Self
    where
        MoveFn: Fn(PlayerIndex<P>, M) -> bool + 'static,
        PayoffFn: Fn(Profile<M, P>) -> Payoff<U, P> + 'static,
    {
        Simultaneous {
            move_fn: Box::new(move_fn),
            payoff_fn: Rc::new(payoff_fn),
        }
    }

    /// Construct a new simultaneous move game given (1) a predicate that determines if a move is
    /// valid for a given player, and (2) a utility function for each player.
    pub fn from_utility_fns<MoveFn, UtilFn>(move_fn: MoveFn, util_fns: PerPlayer<UtilFn, P>) -> Self
    where
        MoveFn: Fn(PlayerIndex<P>, M) -> bool + 'static,
        UtilFn: Fn(M) -> U + 'static,
    {
        let payoff_fn = move |profile: Profile<M, P>| {
            Payoff::new(PerPlayer::generate(|player| {
                util_fns[player](profile[player])
            }))
        };
        Self::from_payoff_fn(move_fn, payoff_fn)
    }

    /// Is this a valid move for the given player?
    pub fn is_valid_move_for_player(&self, player: PlayerIndex<P>, the_move: M) -> bool {
        (*self.move_fn)(player, the_move)
    }

    /// Is this a valid strategy profile? A profile is valid if each move is valid for the
    /// corresponding player.
    pub fn is_valid_profile(&self, profile: Profile<M, P>) -> bool {
        PlayerIndex::all().all(|player| self.is_valid_move_for_player(player, profile[player]))
    }

    /// Get the payoff for the given strategy profile.
    ///
    /// This method may return an arbitrary payoff if given an
    /// [invalid profile](crate::Simultaneous::is_valid_profile).
    pub fn payoff(&self, profile: Profile<M, P>) -> Payoff<U, P> {
        (*self.payoff_fn)(profile)
    }
}
