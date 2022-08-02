//! Games represented in normal form. Simultaneous move games with finite move sets.

mod outcome;
mod profile;
mod solution;
pub use outcome::*;
pub use profile::*;
pub use solution::*;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::iter::Iterator;
use std::rc::Rc;

use crate::core::*;

/// A finite simultaneous-move game represented in [normal form](https://en.wikipedia.org/wiki/Normal-form_game).
///
/// In a normal-form game, each player plays a single move from a finite set of available moves,
/// without knowledge of other players' moves, and the payoff is determined by refering to a table
/// of possible outcomes.
///
/// # Type variables
///
/// - `Move` -- The type of moves played during the game.
/// - `Util` -- The type of utility value awarded to each player in a payoff.
/// - `N` -- The number of players that play the game.
///
/// # Examples
pub struct Normal<Move, Util, const N: usize> {
    moves: PerPlayer<Vec<Move>, N>,
    payoff_fn: Rc<dyn Fn(Profile<Move, N>) -> Payoff<Util, N>>,
}

impl<Move: IsMove, Util: IsUtility, const N: usize> Normal<Move, Util, N> {
    /// Construct a normal-form game given the moves available to each player and a function that
    /// yields the game's payoff given a profile containing a move played by each player.
    ///
    /// If passed an [invalid profile](Normal::is_valid_profile), the payoff function should
    /// return an arbitrary payoff (rather than, say, panic).
    pub fn from_payoff_fn(
        moves: PerPlayer<Vec<Move>, N>,
        payoff_fn: impl Fn(Profile<Move, N>) -> Payoff<Util, N> + 'static,
    ) -> Self {
        Normal {
            moves,
            payoff_fn: Rc::new(payoff_fn),
        }
    }

    /// Construct a normal-form game given the moves available to each player and a map containing
    /// the payoff associated with each valid profile.
    ///
    /// The resulting game will yield a [zero payoff](crate::core::Payoff::zeros) for any profile
    /// not contained in the map.
    pub fn from_payoff_map(
        moves: PerPlayer<Vec<Move>, N>,
        payoff_map: HashMap<Profile<Move, N>, Payoff<Util, N>>,
    ) -> Self {
        let payoff_fn = move |profile| {
            payoff_map
                .get(&profile)
                .copied()
                .unwrap_or_else(|| Payoff::zeros())
        };
        Self::from_payoff_fn(moves, payoff_fn)
    }

    /// Construct a normal-form game given the moves available to each player and a vector of
    /// payoffs in [row-major order](https://en.wikipedia.org/wiki/Row-_and_column-major_order).
    ///
    /// # Errors
    ///
    /// This constructor expects the length of the payoff vector to match the number of profiles
    /// that can be generated from the available moves.
    ///
    /// - If *too few* payoffs are provided, logs an error and returns `None`.
    /// - If *too many* payoffs are provided, logs a warning and returns a table in which the
    ///   excess payoffs are ignored.
    ///
    /// # Examples
    pub fn from_payoff_vec(
        moves: PerPlayer<Vec<Move>, N>,
        payoffs: Vec<Payoff<Util, N>>,
    ) -> Option<Self> {
        let profiles: Vec<Profile<Move, N>> = ProfileIter::from_move_vecs(moves.clone()).collect();
        let num_profiles = profiles.len();
        let num_payoffs = payoffs.len();
        match num_profiles.cmp(&num_payoffs) {
            Ordering::Greater => {
                log::error!(
                    "Normal::new: not enough payoffs provided; expected {}, got {}",
                    num_profiles,
                    num_payoffs,
                );
                return None;
            }
            Ordering::Less => {
                log::warn!(
                    "Normal::new: too many payoffs provided; expected {}, got {}",
                    num_profiles,
                    num_payoffs,
                );
            }
            Ordering::Equal => {}
        }
        let mut payoff_map = HashMap::with_capacity(profiles.len());
        for (profile, payoff) in profiles.into_iter().zip(payoffs) {
            payoff_map.insert(profile, payoff);
        }
        Some(Self::from_payoff_map(moves, payoff_map))
    }

    /// Construct a [symmetric](https://en.wikipedia.org/wiki/Symmetric_game) normal-form game.
    ///
    /// A symmetric game is the same from the perspective of every player.
    ///
    /// The game is constructed from a list of available moves and a vector of utility values for
    /// player `P0` in [row-major order](https://en.wikipedia.org/wiki/Row-_and_column-major_order).
    ///
    /// # Examples
    ///
    /// The classic [prisoner's dilemma](https://en.wikipedia.org/wiki/Prisoner%27s_dilemma) is an
    /// example of a symmetric 2-player game:
    /// ```
    /// use tft::core::*;
    /// use tft::norm::*;
    ///
    /// let pd = Normal::symmetric(
    ///     vec!['C', 'D'],
    ///     vec![2, 0, 3, 1],
    /// ).unwrap();
    ///
    /// assert_eq!(pd.payoff(PerPlayer::new(['C', 'C'])), Payoff::from([2, 2]));
    /// assert_eq!(pd.payoff(PerPlayer::new(['C', 'D'])), Payoff::from([0, 3]));
    /// assert_eq!(pd.payoff(PerPlayer::new(['D', 'C'])), Payoff::from([3, 0]));
    /// assert_eq!(pd.payoff(PerPlayer::new(['D', 'D'])), Payoff::from([1, 1]));
    /// ```
    ///
    /// Symmetric games can be more than two players. Here's an example of a
    /// [3-player prisoner's dilemma](https://classes.cs.uchicago.edu/archive/1998/fall/CS105/Project/node6.html),
    /// where each player's moves and payoffs are symmetric:
    ///
    /// ```
    /// use tft::core::*;
    /// use tft::norm::*;
    ///
    /// let pd3 = Normal::symmetric(
    ///     vec!['C', 'D'],
    ///     vec![4, 1, 1, 0, 5, 3, 3, 2],
    /// ).unwrap();
    ///
    /// assert_eq!(pd3.payoff(PerPlayer::new(['C', 'C', 'C'])), Payoff::from([4, 4, 4]));
    /// assert_eq!(pd3.payoff(PerPlayer::new(['C', 'C', 'D'])), Payoff::from([1, 1, 5]));
    /// assert_eq!(pd3.payoff(PerPlayer::new(['C', 'D', 'C'])), Payoff::from([1, 5, 1]));
    /// assert_eq!(pd3.payoff(PerPlayer::new(['C', 'D', 'D'])), Payoff::from([0, 3, 3]));
    /// assert_eq!(pd3.payoff(PerPlayer::new(['D', 'C', 'C'])), Payoff::from([5, 1, 1]));
    /// assert_eq!(pd3.payoff(PerPlayer::new(['D', 'C', 'D'])), Payoff::from([3, 0, 3]));
    /// assert_eq!(pd3.payoff(PerPlayer::new(['D', 'D', 'C'])), Payoff::from([3, 3, 0]));
    /// assert_eq!(pd3.payoff(PerPlayer::new(['D', 'D', 'D'])), Payoff::from([2, 2, 2]));
    /// ```
    ///
    /// And similarly, a 4-player prisoner's dilemma:
    ///
    /// ```
    /// use tft::core::*;
    /// use tft::norm::*;
    ///
    /// let pd4 = Normal::symmetric(
    ///     vec!['C', 'D'],
    ///     vec![6, 2, 2, 1, 2, 1, 1, 0, 7, 5, 5, 4, 5, 4, 4, 3],
    /// ).unwrap();
    ///
    /// assert_eq!(pd4.payoff(PerPlayer::new(['C', 'C', 'C', 'C'])), Payoff::from([6, 6, 6, 6]));
    /// assert_eq!(pd4.payoff(PerPlayer::new(['C', 'C', 'C', 'D'])), Payoff::from([2, 2, 2, 7]));
    /// assert_eq!(pd4.payoff(PerPlayer::new(['C', 'C', 'D', 'C'])), Payoff::from([2, 2, 7, 2]));
    /// assert_eq!(pd4.payoff(PerPlayer::new(['C', 'C', 'D', 'D'])), Payoff::from([1, 1, 5, 5]));
    /// assert_eq!(pd4.payoff(PerPlayer::new(['C', 'D', 'C', 'C'])), Payoff::from([2, 7, 2, 2]));
    /// assert_eq!(pd4.payoff(PerPlayer::new(['C', 'D', 'C', 'D'])), Payoff::from([1, 5, 1, 5]));
    /// assert_eq!(pd4.payoff(PerPlayer::new(['C', 'D', 'D', 'C'])), Payoff::from([1, 5, 5, 1]));
    /// assert_eq!(pd4.payoff(PerPlayer::new(['C', 'D', 'D', 'D'])), Payoff::from([0, 4, 4, 4]));
    /// assert_eq!(pd4.payoff(PerPlayer::new(['D', 'C', 'C', 'C'])), Payoff::from([7, 2, 2, 2]));
    /// assert_eq!(pd4.payoff(PerPlayer::new(['D', 'C', 'C', 'D'])), Payoff::from([5, 1, 1, 5]));
    /// assert_eq!(pd4.payoff(PerPlayer::new(['D', 'C', 'D', 'C'])), Payoff::from([5, 1, 5, 1]));
    /// assert_eq!(pd4.payoff(PerPlayer::new(['D', 'C', 'D', 'D'])), Payoff::from([4, 0, 4, 4]));
    /// assert_eq!(pd4.payoff(PerPlayer::new(['D', 'D', 'C', 'C'])), Payoff::from([5, 5, 1, 1]));
    /// assert_eq!(pd4.payoff(PerPlayer::new(['D', 'D', 'C', 'D'])), Payoff::from([4, 4, 0, 4]));
    /// assert_eq!(pd4.payoff(PerPlayer::new(['D', 'D', 'D', 'C'])), Payoff::from([4, 4, 4, 0]));
    /// assert_eq!(pd4.payoff(PerPlayer::new(['D', 'D', 'D', 'D'])), Payoff::from([3, 3, 3, 3]));
    /// ```
    #[allow(clippy::needless_range_loop)]
    pub fn symmetric(moves: Vec<Move>, utils: Vec<Util>) -> Option<Self> {
        let num_moves = moves.len();
        let size = num_moves.pow(N as u32);
        let num_utils = utils.len();
        match size.cmp(&num_utils) {
            Ordering::Greater => {
                log::error!(
                    "Normal::symmetric: not enough utility values provided; expected {}^{}={}, got {}",
                    num_moves,
                    N,
                    size,
                    num_utils,
                );
                return None;
            }
            Ordering::Less => {
                log::warn!(
                    "Normal::symmetric: too many utility values provided; expected {}^{}={}, got {}",
                    num_moves,
                    N,
                    size,
                    num_utils,
                );
            }
            Ordering::Equal => {}
        }

        // map that gives the index corresponding to each move
        let mut move_index_map = HashMap::with_capacity(num_moves);
        for (i, m) in moves.clone().into_iter().enumerate() {
            move_index_map.insert(m, i);
        }

        // vector used to translate a profile's move indexes into an index that retrieves player
        // P0's utility from the payoff vector
        let mut translate_p0 = [0; N];
        for i in 0..N {
            translate_p0[i] = num_moves.pow((N - 1 - i) as u32);
        }

        // vectors as above, but for all N players
        let mut translate = [[0; N]; N];
        for p in 0..N {
            for i in 0..N {
                translate[p][i] = translate_p0[(N + i - p) % N];
            }
        }

        // payoff function
        let payoff_fn = move |profile: Profile<Move, N>| {
            // get the profile's move indexes
            let mut move_indexes = [0; N];
            for p in PlayerIndex::all_indexes() {
                let the_move = profile[p];
                if let Some(i) = move_index_map.get(&the_move).copied() {
                    move_indexes[p.as_usize()] = i;
                } else {
                    log::error!(
                        "Normal::symmetric: payoff function received in invalid move: {:?}",
                        the_move
                    );
                }
            }

            let mut payoff_utils = [Util::zero(); N];
            for p in 0..N {
                // compute dot product of translation vector and profile's move indexes to get
                // index into the utility vector
                let util_index: usize = translate[p]
                    .iter()
                    .zip(move_indexes)
                    .map(|(t, i)| t * i)
                    .sum();
                payoff_utils[p] = utils[util_index];
            }
            Payoff::from(payoff_utils)
        };

        Some(Normal::from_payoff_fn(
            PerPlayer::init_with(moves),
            payoff_fn,
        ))
    }

    /// The number of players this game is for.
    pub fn num_players(&self) -> usize {
        N
    }

    /// Get an iterator over the available moves for the given player.
    pub fn available_moves_for_player(&self, player: PlayerIndex<N>) -> MoveIter<Move> {
        MoveIter::new(self.moves[player].clone().into_iter())
    }

    /// Get iterators for moves available to each player.
    pub fn available_moves(&self) -> PerPlayer<MoveIter<Move>, N> {
        PerPlayer::generate(|player| self.available_moves_for_player(player))
    }

    /// Is this a valid move for the given player?
    pub fn is_valid_move(&self, player: PlayerIndex<N>, the_move: Move) -> bool {
        self.moves[player].contains(&the_move)
    }

    /// Is this a valid strategy profile?
    ///
    /// A profile is valid if each move is valid for the corresponding player.
    pub fn is_valid_profile(&self, profile: Profile<Move, N>) -> bool {
        PlayerIndex::all_indexes().all(|pi| self.is_valid_move(pi, profile[pi]))
    }

    /// Get the payoff for the given strategy profile.
    ///
    /// This method should only be called with [valid](Normal::is_valid_profile) profiles.
    /// For invalid profiles, this method will return an arbitrary payoff.
    pub fn payoff(&self, profile: Profile<Move, N>) -> Payoff<Util, N> {
        (*self.payoff_fn)(profile)
    }

    /// An iterator over all of the valid pure strategy profiles for this game.
    pub fn profiles(&self) -> ProfileIter<Move, N> {
        ProfileIter::from_move_iters(self.available_moves())
    }

    /// An iterator over all possible outcomes of the game.
    pub fn outcomes(&self) -> OutcomeIter<Move, Util, N> {
        OutcomeIter::new(self.profiles(), self.payoff_fn.clone())
    }

    /// Is this game zero-sum? In a zero-sum game, the utility values of each payoff sum to zero.
    ///
    /// # Examples
    /// ```
    /// use tft::core::*;
    /// use tft::norm::*;
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
    pub fn is_zero_sum(&self) -> bool {
        self.outcomes().all(|outcome| outcome.payoff.is_zero_sum())
    }

    /// Return a move that unilaterally improves the given player's utility, if such a move exists.
    ///
    /// A unilateral improvement assumes that all other player's moves will be unchanged.
    ///
    /// # Examples
    /// ```
    /// use tft::core::*;
    /// use tft::norm::*;
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
    pub fn unilaterally_improve(
        &self,
        player: PlayerIndex<N>,
        profile: Profile<Move, N>,
    ) -> Option<Move> {
        let mut best_move = None;
        let mut best_util = self.payoff(profile)[player];
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
    /// use tft::core::*;
    /// use tft::norm::*;
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
    pub fn is_stable(&self, profile: Profile<Move, N>) -> bool {
        PlayerIndex::all_indexes()
            .all(|player| self.unilaterally_improve(player, profile).is_none())
    }

    /// All pure Nash equilibrium solutions of a finite simultaneous game.
    ///
    /// # Examples
    /// ```
    /// use tft::core::*;
    /// use tft::norm::*;
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
    pub fn pure_nash_equilibria(&self) -> Vec<Profile<Move, N>> {
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
    /// See the documentation for [`Dominated`](crate::norm::Dominated) for more info.
    ///
    /// # Examples
    /// ```
    /// use tft::core::*;
    /// use tft::norm::*;
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
    pub fn dominated_moves_for(&self, player: PlayerIndex<N>) -> Vec<Dominated<Move>> {
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
    pub fn dominated_moves(&self) -> PerPlayer<Vec<Dominated<Move>>, N> {
        PerPlayer::generate(|index| self.dominated_moves_for(index))
    }
}

impl<Move: IsMove, Util: IsUtility> Normal<Move, Util, 2> {
    /// Construct a matrix game, a two-player zero-sum game where the payoffs are defined by a
    /// single matrix of utility values.
    ///
    /// Constructed from the list of moves for each player and the matrix (in row major order) of
    /// utility values for player `P0`.
    ///
    /// # Examples
    /// ```
    /// use tft::core::*;
    /// use tft::norm::*;
    ///
    /// let g = Normal::matrix(
    ///     vec!['A', 'B', 'C'],
    ///     vec!['D', 'E'],
    ///     vec![-3, -1, 0, 2, 4, 6],
    /// ).unwrap();
    ///
    /// assert!(g.is_zero_sum());
    /// assert_eq!(g.payoff(PerPlayer::new(['A', 'D'])), Payoff::from([-3, 3]));
    /// assert_eq!(g.payoff(PerPlayer::new(['A', 'E'])), Payoff::from([-1, 1]));
    /// assert_eq!(g.payoff(PerPlayer::new(['B', 'D'])), Payoff::from([0, 0]));
    /// assert_eq!(g.payoff(PerPlayer::new(['B', 'E'])), Payoff::from([2, -2]));
    /// assert_eq!(g.payoff(PerPlayer::new(['C', 'D'])), Payoff::from([4, -4]));
    /// assert_eq!(g.payoff(PerPlayer::new(['C', 'E'])), Payoff::from([6, -6]));
    /// ```
    pub fn matrix(
        p0_moves: Vec<Move>,
        p1_moves: Vec<Move>,
        p0_utils: Vec<Util>,
    ) -> Option<Normal<Move, Util, 2>> {
        let moves = PerPlayer::new([p0_moves, p1_moves]);
        let mut payoffs = Vec::with_capacity(p0_utils.len());
        for u0 in p0_utils.into_iter() {
            payoffs.push(Payoff::from([u0, Util::zero().sub(u0)]));
        }
        Normal::from_payoff_vec(moves, payoffs)
    }

    /// Construct a [bimatrix game](https://en.wikipedia.org/wiki/Bimatrix_game), a two-player
    /// game where the payoffs are defined by two matrices of utilities, one for each player.
    ///
    /// Constructed from the list of moves and the matrix (in row major order) of utility values
    /// for each player.
    ///
    /// # Examples
    /// ```
    /// use tft::core::*;
    /// use tft::norm::*;
    ///
    /// let g = Normal::bimatrix(
    ///     vec!['A', 'B', 'C'],
    ///     vec!['D', 'E'],
    ///     vec![0, 5, 4, 3, 2, 1],
    ///     vec![5, 0, 1, 2, 4, 3],
    /// ).unwrap();
    ///
    /// assert_eq!(g.payoff(PerPlayer::new(['A', 'D'])), Payoff::from([0, 5]));
    /// assert_eq!(g.payoff(PerPlayer::new(['A', 'E'])), Payoff::from([5, 0]));
    /// assert_eq!(g.payoff(PerPlayer::new(['B', 'D'])), Payoff::from([4, 1]));
    /// assert_eq!(g.payoff(PerPlayer::new(['B', 'E'])), Payoff::from([3, 2]));
    /// assert_eq!(g.payoff(PerPlayer::new(['C', 'D'])), Payoff::from([2, 4]));
    /// assert_eq!(g.payoff(PerPlayer::new(['C', 'E'])), Payoff::from([1, 3]));
    /// ```
    pub fn bimatrix(
        p0_moves: Vec<Move>,
        p1_moves: Vec<Move>,
        p0_utils: Vec<Util>,
        p1_utils: Vec<Util>,
    ) -> Option<Normal<Move, Util, 2>> {
        let moves = PerPlayer::new([p0_moves, p1_moves]);
        let mut payoffs = Vec::with_capacity(p0_utils.len());
        for (u0, u1) in p0_utils.into_iter().zip(p1_utils) {
            payoffs.push(Payoff::from([u0, u1]));
        }
        Normal::from_payoff_vec(moves, payoffs)
    }
}
