//! Games represented in normal form. Simultaneous move games with finite move sets.

// pub(crate) mod bimatrix;
pub(crate) mod normal;
pub(crate) mod outcome;
pub(crate) mod solution;

pub use crate::game::sim::*;
pub use normal::*;
pub use outcome::*;
pub use solution::*;

use num::Zero;
use std::cmp::Ordering;

use crate::core::*;

/// A [simultaneous game](https://en.wikipedia.org/wiki/Simultaneous_game) where each player can
/// choose from among a finite set of moves.
pub trait IsNormal<const N: usize>: IsSimultaneous<N> {
    /// Get an iterator over the available moves for the given player.
    ///
    /// Implementations of this method should produce every valid move for the given player exactly
    /// once.
    fn available_moves_for_player(&self, player: PlayerIndex<N>) -> MoveIter<'_, Self::Move>;

    /// Get iterators for moves available to each player.
    fn available_moves(&self) -> PerPlayer<MoveIter<'_, Self::Move>, N> {
        PerPlayer::generate(|player| self.available_moves_for_player(player))
    }

    /// Get the number of moves available to each player, which corresponds to the dimensions of
    /// the payoff matrix.
    fn dimensions(&self) -> PerPlayer<usize, N> {
        self.available_moves().map(|ms| ms.count())
    }

    /// An iterator over all of the [valid](Sim::is_valid_profile) pure strategy profiles for this
    /// game.
    fn profiles(&self) -> ProfileIter<'_, Self::Move, N> {
        ProfileIter::from_move_iters(self.available_moves())
    }

    /// An iterator over all possible outcomes of the game.
    fn outcomes(&self) -> OutcomeIter<'_, Self::Move, Self::Util, N> {
        OutcomeIter::for_game(self)
    }

    /// Is this game zero-sum? In a zero-sum game, the utility values of each payoff sum to zero.
    ///
    /// # Examples
    /// ```
    /// use tft::core::*;
    /// use tft::game::norm::*;
    ///
    /// let rps: Normal<_, _, 2> = Normal::symmetric(
    ///     vec!["Rock", "Paper", "Scissors"],
    ///     vec![0, -1, 1, 1, 0, -1, -1, 1, 0],
    /// ).unwrap();
    ///
    /// assert!(rps.is_zero_sum());
    ///
    /// let pd: Normal<_, _, 2> = Normal::symmetric(
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
    /// If more than one move would unilaterally improve the player's utility, then the move that
    /// improves it by the *most* is returned.
    ///
    /// # Examples
    /// ```
    /// use tft::core::*;
    /// use tft::game::norm::*;
    ///
    /// #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
    /// enum RPS { Rock, Paper, Scissors };
    ///
    /// let rps = Normal::symmetric(
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
        if self.is_valid_profile(profile) {
            let mut best_util = self.payoff(profile)[player];
            for adjacent in self.outcomes().adjacent(player, profile) {
                let util = adjacent.payoff[player];
                if util > best_util {
                    best_move = Some(adjacent.profile[player]);
                    best_util = util;
                }
            }
            best_move
        } else {
            log::error!(
                "IsNormal::unilaterally_improve: invalid initial profile ({:?})",
                profile,
            );
            None
        }
    }

    /// Is the given strategy profile stable? A profile is stable if no player can unilaterally
    /// improve their utility.
    ///
    /// A stable profile is a pure
    /// [Nash equilibrium](https://en.wikipedia.org/wiki/Nash_equilibrium) of the game.
    ///
    /// # Examples
    /// ```
    /// use tft::core::*;
    /// use tft::game::norm::*;
    ///
    /// let dilemma = Normal::symmetric(
    ///     vec!['C', 'D'],
    ///     vec![2, 0, 3, 1],
    /// ).unwrap();
    ///
    /// let hunt = Normal::symmetric(
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

    /// All pure [Nash equilibria](https://en.wikipedia.org/wiki/Nash_equilibrium) solutions of a
    /// finite simultaneous game.
    ///
    /// This function simply enumerates all profiles and checks to see if each one is
    /// [stable](Normal::is_stable).
    ///
    /// # Examples
    /// ```
    /// use tft::core::*;
    /// use tft::game::norm::*;
    ///
    /// let dilemma = Normal::symmetric(
    ///     vec!['C', 'D'],
    ///     vec![2, 0, 3, 1],
    /// ).unwrap();
    ///
    /// let hunt = Normal::symmetric(
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

    fn pareto_improve(&self, profile: Profile<Self::Move, N>) -> Option<Profile<Self::Move, N>> {
        if self.is_valid_profile(profile) {
            let payoff = self.payoff(profile);
            let mut best_profile = None;
            let mut best_improvement = <Self::Util as Zero>::zero();
            for outcome in self.outcomes() {
                if let Some(improvement) = payoff.pareto_improvement(outcome.payoff) {
                    if improvement.gt(&best_improvement) {
                        best_profile = Some(outcome.profile);
                        best_improvement = improvement;
                    }
                }
            }
            best_profile
        } else {
            log::error!(
                "IsNormal::pareto_improve: invalid initial profile ({:?})",
                profile,
            );
            None
        }
    }

    fn is_pareto_optimal(&self, profile: Profile<Self::Move, N>) -> bool {
        self.pareto_improve(profile).is_none()
    }

    fn pareto_optimal_solutions(&self) -> Vec<Profile<Self::Move, N>> {
        let mut pareto = Vec::new();
        for profile in self.profiles() {
            if self.is_pareto_optimal(profile) {
                pareto.push(profile)
            }
        }
        pareto
    }

    /// Get all dominated move relationships for the given player. If a move is dominated by
    /// multiple different moves, it will contain multiple entries in the returned vector.
    ///
    /// See the documentation for [`Dominated`](crate::norm::Dominated) for more info.
    ///
    /// # Examples
    /// ```
    /// use tft::core::*;
    /// use tft::game::norm::*;
    ///
    /// let g = Normal::from_payoff_vec(
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
                    if let Some(ordering) = ted_payoff[player].partial_cmp(&tor_payoff[player]) {
                        match ordering {
                            Ordering::Less => {}
                            Ordering::Equal => is_strict = false,
                            Ordering::Greater => {
                                is_dominated = false;
                                break;
                            }
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
