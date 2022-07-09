//! Defines traits over various classes of games.

use num::Num;
use std::cmp::Ordering;
use std::fmt::Debug;
use std::hash::Hash;
use std::iter::Iterator;

use crate::core::{OutcomeIter, Payoff, PerPlayer, PlayerIndex, Profile, ProfileIter};
use crate::solution::Dominated;

/// The most general trait for games. Includes associated types and methods that all games must
/// support.
///
/// The const type variable `N` indicates the number of players this game is for.
pub trait Game<const N: usize> {
    /// The type of moves played during the game.
    type Move: Copy + Debug + Eq + Hash;

    /// The type of utility value awarded to each player in the payoff at the end of the game.
    type Utility: Copy + Debug + Num + Ord;

    /// The type of state maintained while executing an iteration of this game.
    type State: Clone;

    /// Get the initial execution state for this game.
    fn initial_state(&self) -> Self::State;

    /// Is this a valid move for the given player at the given execution state?
    fn is_valid_move_for_player_at_state(
        &self,
        player: PlayerIndex<N>,
        state: &Self::State,
        the_move: Self::Move,
    ) -> bool;

    /// The number of players this game is for.
    fn num_players(&self) -> usize {
        N
    }
}

/// A game with a finite set of available moves at each decision point.
pub trait Finite<const N: usize>: Game<N> {
    /// An iterator over available moves.
    type MoveIter: Clone + Iterator<Item = Self::Move>;

    /// Get the set of moves available at the given execution state.
    fn available_moves_for_player_at_state(
        &self,
        player: PlayerIndex<N>,
        state: &Self::State,
    ) -> Self::MoveIter;
}

/// A game in which each player plays a single move without knowledge of the other players' moves.
///
/// Since simultaneous games consist of only a single simultaneous move, they have a trivial
/// execution state of type `()`.
pub trait Simultaneous<const N: usize>: Game<N, State = ()> {
    /// Get the payoff for a given strategy profile.
    ///
    /// # Errors
    /// *May* return `None` if the profile is invalid (i.e. contains an invalid move for some
    /// player). However, implementors are not required to check the validity of the profile, and
    /// may return a (meaningless) payoff for an invalid profile.
    fn payoff(&self, profile: Profile<Self::Move, N>) -> Option<Payoff<Self::Utility, N>>;

    /// Is this a valid move for the given player?
    fn is_valid_move_for_player(&self, player: PlayerIndex<N>, the_move: Self::Move) -> bool {
        self.is_valid_move_for_player_at_state(player, &(), the_move)
    }

    /// Is the given strategy profile valid?
    ///
    /// A profile is valid if each move is valid for the corresponding player.
    fn is_valid_profile(&self, profile: Profile<Self::Move, N>) -> bool {
        PlayerIndex::all_indexes().all(|pi| self.is_valid_move_for_player(pi, profile[pi]))
    }

    /// If this profile is valid, does it yield a profile?
    ///
    /// Returns a [vacuous](https://en.wikipedia.org/wiki/Vacuous_truth) `true` if the profile is
    /// invalid.
    ///
    /// The [`Simultaneous::payoff`] method should yield a payoff for every valid profile. This
    /// function checks whether this property holds for a given profile and is intended for use in
    /// tests.
    fn law_valid_profile_yields_payoff(&self, profile: Profile<Self::Move, N>) -> bool {
        if self.is_valid_profile(profile) {
            self.payoff(profile).is_some()
        } else {
            true // vacuously
        }
    }
}

/// A game that is both finite and simultaneous, such as games in normal form.
///
/// Each player plays a single move from a finite set of available moves, without knowledge of
/// other players' moves.
///
/// Note that many of this trait's default method implementations are naive algorithms that iterate
/// over all of the outcomes of the game. Implementors of this trait should provide more efficient
/// implementations, where possible.
pub trait FiniteSimultaneous<const N: usize>: Finite<N> + Simultaneous<N> {
    /// Iterate over the moves available to the given player.
    fn available_moves_for_player(&self, player: PlayerIndex<N>) -> Self::MoveIter {
        self.available_moves_for_player_at_state(player, &())
    }

    /// Get iterators for moves available to each player.
    fn available_moves(&self) -> PerPlayer<Self::MoveIter, N> {
        PerPlayer::generate(|player| self.available_moves_for_player(player))
    }

    /// An iterator over all of the valid pure strategy profiles for this game.
    fn profiles(&self) -> ProfileIter<Self::Move, Self::MoveIter, N> {
        ProfileIter::from_move_iters(self.available_moves())
    }

    /// An iterator over all possible outcomes of the game.
    fn outcomes(&self) -> OutcomeIter<Self::Move, Self::MoveIter, Self::Utility, N> {
        let payoff_fn = |profile| {
            if let Some(payoff) = self.payoff(profile) {
                payoff
            } else {
                log::error!(
                    "Normal::outcomes: payoff function return `None` for profile {:?}",
                    profile
                );
                Payoff::zeros()
            }
        };
        OutcomeIter::new(self.profiles(), payoff_fn)
    }

    /// Do all of the profiles returned by [`FiniteSimultaneous::profiles`] yield payoffs?
    ///
    /// This function checks whether this property holds and is intended for use in tests.
    fn law_all_valid_profiles_yield_payoffs(&self) -> bool {
        self.profiles().all(|p| self.payoff(p).is_some())
    }

    /// Is this game zero-sum? In a zero-sum game, the utility values of each payoff sum to zero.
    ///
    /// # Examples
    /// ```
    /// use tft::prelude::*;
    /// use tft::game::Normal;
    ///
    /// let rps = Normal::symmetric_for2(
    ///     vec!["Rock", "Paper", "Scissors"],
    ///     vec![0, -1, 1, 1, 0, -1, -1, 1, 0],
    /// ).unwrap();
    ///
    /// assert!(rps.is_zero_sum());
    ///
    /// let pd = Normal::symmetric_for2(
    ///     vec!["Cooperate", "Defect"],
    ///     vec![2, 0, 3, 1],
    /// ).unwrap();
    ///
    /// assert!(!pd.is_zero_sum());
    /// ```
    fn is_zero_sum(&self) -> bool {
        self.outcomes().all(|outcome| outcome.payoff.is_zero_sum())
    }

    /// Return a move that unilaterally improves the given player's utility, if such a move exists.
    ///
    /// A unilateral improvement assumes that all other player's moves will be unchanged.
    ///
    /// # Examples
    /// ```
    /// use tft::prelude::*;
    /// use tft::game::Normal;
    ///
    /// #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
    /// enum RPS { Rock, Paper, Scissors };
    ///
    /// let rps = Normal::symmetric_for2(
    ///     vec![RPS::Rock, RPS::Paper, RPS::Scissors],
    ///     vec![ 0, -1,  1,
    ///           1,  0, -1,
    ///          -1,  1,  0,
    ///     ],
    /// ).unwrap();
    ///
    /// let rock_rock = PerPlayer::new([RPS::Rock, RPS::Rock]);
    /// assert_eq!(rps.unilaterally_improve(for2::P0, rock_rock), Some(RPS::Paper));
    /// assert_eq!(rps.unilaterally_improve(for2::P1, rock_rock), Some(RPS::Paper));
    ///
    /// let paper_scissors = PerPlayer::new([RPS::Paper, RPS::Scissors]);
    /// assert_eq!(rps.unilaterally_improve(for2::P0, paper_scissors), Some(RPS::Rock));
    /// assert_eq!(rps.unilaterally_improve(for2::P1, paper_scissors), None);
    ///
    /// let paper_rock = PerPlayer::new([RPS::Paper, RPS::Rock]);
    /// assert_eq!(rps.unilaterally_improve(for2::P0, paper_rock), None);
    /// assert_eq!(rps.unilaterally_improve(for2::P1, paper_rock), Some(RPS::Scissors));
    /// ```
    fn unilaterally_improve(
        &self,
        player: PlayerIndex<N>,
        profile: Profile<Self::Move, N>,
    ) -> Option<Self::Move> {
        let mut best_move = None;
        let mut best_util = match self.payoff(profile) {
            Some(payoff) => payoff[player],
            None => {
                log::warn!(
                    "Normal::unilaterally_improve(): invalid initial profile: {:?}",
                    profile,
                );
                return best_move;
            }
        };
        for adjacent in self.outcomes().adjacent(player, profile) {
            let util = adjacent.payoff[player];
            if util > best_util {
                best_move = Some(adjacent.profile[player]);
                best_util = util;
            }
        }
        best_move
    }

    /// Is the given strategy profile stable? A profile is stable if no player can unilaterally
    /// improve their utility.
    ///
    /// A stable profile is a pure Nash equilibrium of the game.
    ///
    /// # Examples
    /// ```
    /// use tft::prelude::*;
    /// use tft::game::Normal;
    ///
    /// let dilemma = Normal::symmetric_for2(
    ///     vec!['C', 'D'],
    ///     vec![2, 0, 3, 1],
    /// ).unwrap();
    ///
    /// let hunt = Normal::symmetric_for2(
    ///     vec!['C', 'D'],
    ///     vec![3, 0, 2, 1],
    /// ).unwrap();
    ///
    /// let cc = PerPlayer::new(['C', 'C']);
    /// let cd = PerPlayer::new(['C', 'D']);
    /// let dc = PerPlayer::new(['D', 'C']);
    /// let dd = PerPlayer::new(['D', 'D']);
    ///
    /// assert!(!dilemma.is_stable(cc));
    /// assert!(!dilemma.is_stable(cd));
    /// assert!(!dilemma.is_stable(dc));
    /// assert!(dilemma.is_stable(dd));
    ///
    /// assert!(hunt.is_stable(cc));
    /// assert!(!hunt.is_stable(cd));
    /// assert!(!hunt.is_stable(dc));
    /// assert!(hunt.is_stable(dd));
    /// ```
    fn is_stable(&self, profile: Profile<Self::Move, N>) -> bool {
        PlayerIndex::all_indexes()
            .all(|player| self.unilaterally_improve(player, profile).is_none())
    }

    /// All pure Nash equilibrium solutions of a finite simultaneous game.
    ///
    /// # Examples
    /// ```
    /// use tft::prelude::*;
    /// use tft::game::Normal;
    ///
    /// let dilemma = Normal::symmetric_for2(
    ///     vec!['C', 'D'],
    ///     vec![2, 0, 3, 1],
    /// ).unwrap();
    ///
    /// let hunt = Normal::symmetric_for2(
    ///     vec!['C', 'D'],
    ///     vec![3, 0, 2, 1],
    /// ).unwrap();
    ///
    /// assert_eq!(
    ///     dilemma.pure_nash_equilibria(),
    ///     vec![PerPlayer::new(['D', 'D'])],
    /// );
    /// assert_eq!(
    ///     hunt.pure_nash_equilibria(),
    ///     vec![PerPlayer::new(['C', 'C']), PerPlayer::new(['D', 'D'])],
    /// );
    /// ```
    fn pure_nash_equilibria(&self) -> Vec<Profile<Self::Move, N>> {
        let mut nash = Vec::new();
        for profile in self.profiles() {
            if self.is_stable(profile) {
                nash.push(profile);
            }
        }
        nash
    }

    /// Get all dominated move relationships for the given player. If a move is dominated by
    /// multiple different moves, it will contain multiple entries in the returned vector.
    ///
    /// See the documentation for [`Dominated`](crate::solution::Dominated) for more info.
    ///
    /// # Examples
    /// ```
    /// use tft::prelude::*;
    /// use tft::game::Normal;
    /// use tft::solution::Dominated;
    ///
    /// let g = Normal::new(
    ///     PerPlayer::new([
    ///         vec!['A', 'B', 'C'],
    ///         vec!['D', 'E'],
    ///     ]),
    ///     vec![
    ///         Payoff::from([3, 3]), Payoff::from([3, 5]),
    ///         Payoff::from([2, 0]), Payoff::from([3, 1]),
    ///         Payoff::from([4, 0]), Payoff::from([2, 1]),
    ///     ],
    /// ).unwrap();
    ///
    /// assert_eq!(g.dominated_moves_for(for2::P0), vec![Dominated::weak('B', 'A')]);
    /// assert_eq!(g.dominated_moves_for(for2::P1), vec![Dominated::strict('D', 'E')]);
    /// ```
    fn dominated_moves_for(&self, player: PlayerIndex<N>) -> Vec<Dominated<Self::Move>> {
        let mut dominated = Vec::new();

        for maybe_ted in self.available_moves_for_player(player) {
            let ted_iter = self.outcomes().include(player, maybe_ted);

            for maybe_tor in self.available_moves_for_player(player) {
                if maybe_ted == maybe_tor {
                    continue;
                }

                let tor_iter = self.outcomes().include(player, maybe_tor);

                let mut is_dominated = true;
                let mut is_strict = true;
                for (ted_outcome, tor_outcome) in ted_iter.clone().zip(tor_iter) {
                    let ted_payoff = ted_outcome.payoff;
                    let tor_payoff = tor_outcome.payoff;
                    match ted_payoff[player].cmp(&tor_payoff[player]) {
                        Ordering::Less => {}
                        Ordering::Equal => is_strict = false,
                        Ordering::Greater => {
                            is_dominated = false;
                            break;
                        }
                    }
                }
                if is_dominated {
                    dominated.push(Dominated {
                        dominated: maybe_ted,
                        dominator: maybe_tor,
                        is_strict,
                    });
                }
            }
        }
        dominated
    }

    /// Get all dominated move relationships for each player.
    fn dominated_moves(&self) -> PerPlayer<Vec<Dominated<Self::Move>>, N> {
        PerPlayer::generate(|index| self.dominated_moves_for(index))
    }
}
