use num::Zero;
use rayon::prelude::*;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::iter::Iterator;
use std::sync::Arc;

use crate::{
    Dominated, ErrorKind, Game, Move, Outcome, Payoff, PerPlayer, PlayerIndex, PossibleMoves,
    PossibleOutcomes, PossibleProfiles, Profile, Record, Simultaneous, SimultaneousOutcome, Turn,
    Utility,
};

/// A game represented in [normal form](https://en.wikipedia.org/wiki/Normal-form_game).
///
/// In a normal-form game, each player plays a single move from a finite set of available moves,
/// without knowledge of other players' moves, and the payoff is determined by referring to a table
/// of possible outcomes.
///
/// # Type variables
///
/// - `M` -- The type of moves played during the game.
/// - `U` -- The type of utility value awarded to each player in a payoff.
/// - `P` -- The number of players that play the game.
///
/// # Examples
/// ```
/// use t4t::*;
///
/// let pd = Normal::symmetric(
///     vec!['C', 'D'],
///     vec![2, 0, 3, 1],
/// ).unwrap();
///
/// let nice = Player::new("Nice".to_string(), || Strategy::pure('C'));
/// let mean = Player::new("Mean".to_string(), || Strategy::pure('D'));
///
/// assert_eq!(
///     pd.play(&Matchup::from_players([nice.clone(), nice.clone()])),
///     Ok(SimultaneousOutcome::new(Profile::new(['C', 'C']), Payoff::from([2, 2]))),
/// );
/// assert_eq!(
///     pd.play(&Matchup::from_players([nice.clone(), mean.clone()])),
///     Ok(SimultaneousOutcome::new(Profile::new(['C', 'D']), Payoff::from([0, 3]))),
/// );
/// assert_eq!(
///     pd.play(&Matchup::from_players([mean.clone(), nice])),
///     Ok(SimultaneousOutcome::new(Profile::new(['D', 'C']), Payoff::from([3, 0]))),
/// );
/// assert_eq!(
///     pd.play(&Matchup::from_players([mean.clone(), mean])),
///     Ok(SimultaneousOutcome::new(Profile::new(['D', 'D']), Payoff::from([1, 1]))),
/// );
/// ```
#[derive(Clone)]
pub struct Normal<M, U, const P: usize> {
    moves: PerPlayer<Vec<M>, P>,
    payoff_fn: Arc<dyn Fn(Profile<M, P>) -> Payoff<U, P> + Send + Sync>,
}

impl<M: Move, U: Utility, const P: usize> Game<P> for Normal<M, U, P> {
    type Move = M;
    type Utility = U;
    type Outcome = SimultaneousOutcome<M, U, P>;
    type State = ();
    type View = ();

    fn first_turn(&self) -> Turn<(), M, SimultaneousOutcome<M, U, P>, P> {
        let state = Arc::new(());
        Turn::all_players(state.clone(), move |_, profile| {
            for ply in profile.plies() {
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

impl<M: Move, U: Utility, const P: usize> Normal<M, U, P> {
    /// Construct a normal-form game given the moves available to each player and a function that
    /// yields the game's payoff given a profile containing a move played by each player.
    ///
    /// This constructor (and [from_utility_fns](Normal::from_utility_fns)) enables representing
    /// large normal-form games where it would be intractable to represent the payoff map/table
    /// directly.
    pub fn from_payoff_fn(
        moves: PerPlayer<Vec<M>, P>,
        payoff_fn: impl Fn(Profile<M, P>) -> Payoff<U, P> + Send + Sync + 'static,
    ) -> Self {
        Normal {
            moves,
            payoff_fn: Arc::new(payoff_fn),
        }
    }

    /// Construct a normal-form game given the moves available to each player and a utility
    /// function for each player.
    ///
    /// This constructor (and [from_payoff_fn](Normal::from_payoff_fn)) enables representing
    /// large normal-form games where it would be intractable to represent the payoff map/table
    /// directly.
    pub fn from_utility_fns(
        moves: PerPlayer<Vec<M>, P>,
        util_fns: PerPlayer<impl Fn(M) -> U + Send + Sync + 'static, P>,
    ) -> Self {
        let payoff_fn = move |profile: Profile<M, P>| {
            Payoff::new(PerPlayer::generate(|player| {
                util_fns[player](profile[player])
            }))
        };
        Self::from_payoff_fn(moves, payoff_fn)
    }

    /// Construct a normal-form game given the moves available to each player and a map containing
    /// the payoff associated with each valid profile.
    ///
    /// # Errors
    ///
    /// The resulting game will log an error and return a [zero payoff](Payoff::zeros) for
    /// any profile not contained in the map.
    pub fn from_payoff_map(
        moves: PerPlayer<Vec<M>, P>,
        payoff_map: HashMap<Profile<M, P>, Payoff<U, P>>,
    ) -> Self {
        let payoff_fn = move |profile| {
            if let Some(payoff) = payoff_map.get(&profile).copied() {
                payoff
            } else {
                log::error!(
                    "Normal::from_payoff_map: attempted to get the payoff of a profile not in the map: {:?}",
                    profile
                );
                Payoff::zeros()
            }
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
    /// - If *too many* payoffs are provided, logs a warning and returns a game in which the
    ///   excess payoffs are ignored.
    ///
    /// # Examples
    /// ```
    /// use t4t::*;
    ///
    /// let g = Normal::from_payoff_vec(
    ///     PerPlayer::new([vec!['A', 'B'], vec!['C', 'D'], vec!['E']]),
    ///     vec![
    ///         Payoff::from([1, 2, 3]), Payoff::from([4, 5, 6]),
    ///         Payoff::from([9, 8, 7]), Payoff::from([6, 5, 4]),
    ///     ]
    /// )
    /// .unwrap();
    ///
    /// assert_eq!(g.payoff(Profile::new(['A', 'C', 'E'])), Payoff::from([1, 2, 3]));
    /// assert_eq!(g.payoff(Profile::new(['A', 'D', 'E'])), Payoff::from([4, 5, 6]));
    /// assert_eq!(g.payoff(Profile::new(['B', 'C', 'E'])), Payoff::from([9, 8, 7]));
    /// assert_eq!(g.payoff(Profile::new(['B', 'D', 'E'])), Payoff::from([6, 5, 4]));
    /// ```
    pub fn from_payoff_vec(
        moves: PerPlayer<Vec<M>, P>,
        payoffs: Vec<Payoff<U, P>>,
    ) -> Option<Self> {
        let profiles: Vec<Profile<M, P>> =
            PossibleProfiles::from_move_vecs(moves.clone()).collect();
        let num_profiles = profiles.len();
        let num_payoffs = payoffs.len();
        match num_profiles.cmp(&num_payoffs) {
            Ordering::Greater => {
                log::error!(
                    "Normal::from_payoff_vec: not enough payoffs provided; expected {}, got {}",
                    num_profiles,
                    num_payoffs,
                );
                return None;
            }
            Ordering::Less => {
                log::warn!(
                    "Normal::from_payoff_vec: too many payoffs provided; expected {}, got {}",
                    num_profiles,
                    num_payoffs,
                );
            }
            Ordering::Equal => {}
        }
        let mut payoff_map = HashMap::with_capacity(num_profiles);
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
    /// use t4t::*;
    ///
    /// let pd = Normal::symmetric(
    ///     vec!['C', 'D'],
    ///     vec![2, 0, 3, 1],
    /// ).unwrap();
    ///
    /// assert_eq!(pd.payoff(Profile::new(['C', 'C'])), Payoff::from([2, 2]));
    /// assert_eq!(pd.payoff(Profile::new(['C', 'D'])), Payoff::from([0, 3]));
    /// assert_eq!(pd.payoff(Profile::new(['D', 'C'])), Payoff::from([3, 0]));
    /// assert_eq!(pd.payoff(Profile::new(['D', 'D'])), Payoff::from([1, 1]));
    /// ```
    ///
    /// Symmetric games can be more than two players. Here's an example of a
    /// [3-player prisoner's dilemma](https://classes.cs.uchicago.edu/archive/1998/fall/CS105/Project/node6.html),
    /// where each player's moves and payoffs are symmetric:
    ///
    /// ```
    /// use t4t::*;
    ///
    /// let pd3 = Normal::symmetric(
    ///     vec!['C', 'D'],
    ///     vec![4, 1, 1, 0, 5, 3, 3, 2],
    /// ).unwrap();
    ///
    /// assert_eq!(pd3.payoff(Profile::new(['C', 'C', 'C'])), Payoff::from([4, 4, 4]));
    /// assert_eq!(pd3.payoff(Profile::new(['C', 'C', 'D'])), Payoff::from([1, 1, 5]));
    /// assert_eq!(pd3.payoff(Profile::new(['C', 'D', 'C'])), Payoff::from([1, 5, 1]));
    /// assert_eq!(pd3.payoff(Profile::new(['C', 'D', 'D'])), Payoff::from([0, 3, 3]));
    /// assert_eq!(pd3.payoff(Profile::new(['D', 'C', 'C'])), Payoff::from([5, 1, 1]));
    /// assert_eq!(pd3.payoff(Profile::new(['D', 'C', 'D'])), Payoff::from([3, 0, 3]));
    /// assert_eq!(pd3.payoff(Profile::new(['D', 'D', 'C'])), Payoff::from([3, 3, 0]));
    /// assert_eq!(pd3.payoff(Profile::new(['D', 'D', 'D'])), Payoff::from([2, 2, 2]));
    /// ```
    ///
    /// And similarly, a 4-player prisoner's dilemma:
    ///
    /// ```
    /// use t4t::*;
    ///
    /// let pd4 = Normal::symmetric(
    ///     vec!['C', 'D'],
    ///     vec![6, 2, 2, 1, 2, 1, 1, 0, 7, 5, 5, 4, 5, 4, 4, 3],
    /// ).unwrap();
    ///
    /// assert_eq!(pd4.payoff(Profile::new(['C', 'C', 'C', 'C'])), Payoff::from([6, 6, 6, 6]));
    /// assert_eq!(pd4.payoff(Profile::new(['C', 'C', 'C', 'D'])), Payoff::from([2, 2, 2, 7]));
    /// assert_eq!(pd4.payoff(Profile::new(['C', 'C', 'D', 'C'])), Payoff::from([2, 2, 7, 2]));
    /// assert_eq!(pd4.payoff(Profile::new(['C', 'C', 'D', 'D'])), Payoff::from([1, 1, 5, 5]));
    /// assert_eq!(pd4.payoff(Profile::new(['C', 'D', 'C', 'C'])), Payoff::from([2, 7, 2, 2]));
    /// assert_eq!(pd4.payoff(Profile::new(['C', 'D', 'C', 'D'])), Payoff::from([1, 5, 1, 5]));
    /// assert_eq!(pd4.payoff(Profile::new(['C', 'D', 'D', 'C'])), Payoff::from([1, 5, 5, 1]));
    /// assert_eq!(pd4.payoff(Profile::new(['C', 'D', 'D', 'D'])), Payoff::from([0, 4, 4, 4]));
    /// assert_eq!(pd4.payoff(Profile::new(['D', 'C', 'C', 'C'])), Payoff::from([7, 2, 2, 2]));
    /// assert_eq!(pd4.payoff(Profile::new(['D', 'C', 'C', 'D'])), Payoff::from([5, 1, 1, 5]));
    /// assert_eq!(pd4.payoff(Profile::new(['D', 'C', 'D', 'C'])), Payoff::from([5, 1, 5, 1]));
    /// assert_eq!(pd4.payoff(Profile::new(['D', 'C', 'D', 'D'])), Payoff::from([4, 0, 4, 4]));
    /// assert_eq!(pd4.payoff(Profile::new(['D', 'D', 'C', 'C'])), Payoff::from([5, 5, 1, 1]));
    /// assert_eq!(pd4.payoff(Profile::new(['D', 'D', 'C', 'D'])), Payoff::from([4, 4, 0, 4]));
    /// assert_eq!(pd4.payoff(Profile::new(['D', 'D', 'D', 'C'])), Payoff::from([4, 4, 4, 0]));
    /// assert_eq!(pd4.payoff(Profile::new(['D', 'D', 'D', 'D'])), Payoff::from([3, 3, 3, 3]));
    /// ```
    #[allow(clippy::needless_range_loop)]
    pub fn symmetric(moves: Vec<M>, utils: Vec<U>) -> Option<Self> {
        let num_moves = moves.len();
        let size = num_moves.pow(P as u32);
        let num_utils = utils.len();
        match size.cmp(&num_utils) {
            Ordering::Greater => {
                log::error!(
                    "Normal::symmetric: not enough utility values provided; expected {}^{}={}, got {}",
                    num_moves,
                    P,
                    size,
                    num_utils,
                );
                return None;
            }
            Ordering::Less => {
                log::warn!(
                    "Normal::symmetric: too many utility values provided; expected {}^{}={}, got {}",
                    num_moves,
                    P,
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
        let mut translate_p0 = [0; P];
        for i in 0..P {
            translate_p0[i] = num_moves.pow((P - 1 - i) as u32);
        }

        // vectors as above, but for all P players
        let mut translate = [[0; P]; P];
        for p in 0..P {
            for i in 0..P {
                translate[p][i] = translate_p0[(P + i - p) % P];
            }
        }

        // payoff function
        let payoff_fn = move |profile: Profile<M, P>| {
            // get the profile's move indexes
            let mut move_indexes = [0; P];
            for p in PlayerIndex::all() {
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

            let mut payoff_utils = [U::zero(); P];
            for p in 0..P {
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

    /// Get an iterator over the available moves for the given player.
    pub fn possible_moves_for_player(&self, player: PlayerIndex<P>) -> PossibleMoves<'_, M> {
        PossibleMoves::from_vec(self.moves[player].clone())
    }

    /// Get iterators for the moves available to each player.
    pub fn possible_moves(&self) -> PerPlayer<PossibleMoves<'_, M>, P> {
        PerPlayer::generate(|player| self.possible_moves_for_player(player))
    }

    /// Is this a valid move for the given player?
    pub fn is_valid_move_for_player(&self, player: PlayerIndex<P>, the_move: M) -> bool {
        self.moves[player].contains(&the_move)
    }

    /// Is this a valid strategy profile? A profile is valid if each move is valid for the
    /// corresponding player.
    pub fn is_valid_profile(&self, profile: Profile<M, P>) -> bool {
        PlayerIndex::all().all(|player| self.is_valid_move_for_player(player, profile[player]))
    }

    /// Get the payoff for the given strategy profile.
    ///
    /// This method may return an arbitrary payoff if given an
    /// [invalid profile](Normal::is_valid_profile).
    pub fn payoff(&self, profile: Profile<M, P>) -> Payoff<U, P> {
        (*self.payoff_fn)(profile)
    }

    /// Get the number of moves available to each player, which corresponds to the dimensions of
    /// the payoff matrix.
    pub fn dimensions(&self) -> PerPlayer<usize, P> {
        self.possible_moves().map(|ms| ms.count())
    }

    /// Get this normal form game as a simultaneous move game.
    pub fn as_simultaneous(&self) -> Simultaneous<M, U, P> {
        let moves = self.moves.clone();
        let payoff_fn = self.payoff_fn.clone();
        Simultaneous::from_payoff_fn(
            move |player, the_move| moves[player].contains(&the_move),
            move |profile| payoff_fn(profile),
        )
    }

    /// An iterator over all of the [valid](Normal::is_valid_profile) pure strategy profiles for
    /// this game.
    pub fn possible_profiles(&self) -> PossibleProfiles<'_, M, P> {
        PossibleProfiles::from_move_iters(self.possible_moves())
    }

    /// An iterator over all possible outcomes of the game.
    pub fn possible_outcomes(&self) -> PossibleOutcomes<'_, M, U, P> {
        PossibleOutcomes::new(self.possible_profiles(), self.payoff_fn.clone())
    }

    /// Is this game zero-sum? In a zero-sum game, the utility values of each payoff sum to zero.
    ///
    /// # Examples
    /// ```
    /// use t4t::*;
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
        self.possible_outcomes()
            .all(|outcome| outcome.payoff().is_zero_sum())
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
    /// use t4t::*;
    ///
    /// #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
    /// enum RPS { Rock, Paper, Scissors }
    ///
    /// let rps = Normal::symmetric(
    ///     vec![RPS::Rock, RPS::Paper, RPS::Scissors],
    ///     vec![ 0, -1,  1,
    ///           1,  0, -1,
    ///          -1,  1,  0,
    ///     ],
    /// ).unwrap();
    ///
    /// let rock_rock = Profile::new([RPS::Rock, RPS::Rock]);
    /// assert_eq!(rps.unilaterally_improve(for2::P0, rock_rock), Some(RPS::Paper));
    /// assert_eq!(rps.unilaterally_improve(for2::P1, rock_rock), Some(RPS::Paper));
    ///
    /// let paper_scissors = Profile::new([RPS::Paper, RPS::Scissors]);
    /// assert_eq!(rps.unilaterally_improve(for2::P0, paper_scissors), Some(RPS::Rock));
    /// assert_eq!(rps.unilaterally_improve(for2::P1, paper_scissors), None);
    ///
    /// let paper_rock = Profile::new([RPS::Paper, RPS::Rock]);
    /// assert_eq!(rps.unilaterally_improve(for2::P0, paper_rock), None);
    /// assert_eq!(rps.unilaterally_improve(for2::P1, paper_rock), Some(RPS::Scissors));
    /// ```
    pub fn unilaterally_improve(
        &self,
        player: PlayerIndex<P>,
        profile: Profile<M, P>,
    ) -> Option<M> {
        let mut best_move = None;
        if self.is_valid_profile(profile) {
            let mut best_util = self.payoff(profile)[player];
            for adjacent in self.possible_outcomes().adjacent(player, profile) {
                let util = adjacent.payoff()[player];
                if util > best_util {
                    best_move = Some(adjacent.profile()[player]);
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
    /// use t4t::*;
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
    /// let cc = Profile::new(['C', 'C']);
    /// let cd = Profile::new(['C', 'D']);
    /// let dc = Profile::new(['D', 'C']);
    /// let dd = Profile::new(['D', 'D']);
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
    pub fn is_stable(&self, profile: Profile<M, P>) -> bool {
        PlayerIndex::all().all(|player| self.unilaterally_improve(player, profile).is_none())
    }

    /// All pure [Nash equilibria](https://en.wikipedia.org/wiki/Nash_equilibrium) solutions of a
    /// finite simultaneous game.
    ///
    /// This function simply enumerates all profiles and checks to see if each one is
    /// [stable](Normal::is_stable).
    ///
    /// # Examples
    /// ```
    /// use t4t::*;
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
    ///     vec![Profile::new(['D', 'D'])],
    /// );
    /// assert_eq!(
    ///     hunt.pure_nash_equilibria(),
    ///     vec![Profile::new(['C', 'C']), Profile::new(['D', 'D'])],
    /// );
    /// ```
    pub fn pure_nash_equilibria(&self) -> Vec<Profile<M, P>> {
        let mut nash = Vec::new();
        for profile in self.possible_profiles() {
            if self.is_stable(profile) {
                nash.push(profile);
            }
        }
        nash
    }

    /// A variant of [`pure_nash_equilibria`](Self::pure_nash_equilibria) that analyzes the outcomes
    /// in parallel.
    pub fn pure_nash_equlibria_parallel(&self) -> Vec<Profile<M, P>> {
        let (sender, receiver) = std::sync::mpsc::channel();
        self.possible_profiles()
            .par_bridge()
            .for_each_with(sender, |s, profile| {
                if self.is_stable(profile) {
                    s.send(profile).unwrap();
                }
            });
        receiver.iter().collect()
    }

    /// Return a new profile that represents a
    /// [Pareto improvement](https://en.wikipedia.org/wiki/Pareto_efficiency)
    /// on the given profile, if one exists.
    ///
    /// A profile is a Pareto improvement over another if the payoff associated with the improved
    /// profile *increases the utility for at least one player* over the payoff associated with the
    /// original profile, *without decreasing the utility for any players*.
    pub fn pareto_improve(&self, profile: Profile<M, P>) -> Option<Profile<M, P>> {
        if self.is_valid_profile(profile) {
            let payoff = self.payoff(profile);
            let mut best_profile = None;
            let mut best_improvement = <U as Zero>::zero();
            for outcome in self.possible_outcomes() {
                if let Some(improvement) = payoff.pareto_improvement(*outcome.payoff()) {
                    if improvement.gt(&best_improvement) {
                        best_profile = Some(*outcome.profile());
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

    /// A profile is [Pareto optimal](https://en.wikipedia.org/wiki/Pareto_efficiency) if there is
    /// no other profile that represents a [Pareto improvement](Normal::pareto_improve).
    pub fn is_pareto_optimal(&self, profile: Profile<M, P>) -> bool {
        self.pareto_improve(profile).is_none()
    }

    /// Get all profiles that are [Pareto optimal](Normal::is_pareto_optimal).
    pub fn pareto_optimal_solutions(&self) -> Vec<Profile<M, P>> {
        let mut pareto = Vec::new();
        for profile in self.possible_profiles() {
            if self.is_pareto_optimal(profile) {
                pareto.push(profile)
            }
        }
        pareto
    }

    /// A variant of [`pareto_optimal_solutions`](Self::pareto_optimal_solutions) that analyzes the
    /// outcomes in parallel.
    pub fn pareto_optimal_solutions_parallel(&self) -> Vec<Profile<M, P>> {
        let (sender, receiver) = std::sync::mpsc::channel();
        self.possible_profiles()
            .par_bridge()
            .for_each_with(sender, |s, profile| {
                if self.is_pareto_optimal(profile) {
                    s.send(profile).unwrap();
                }
            });
        receiver.iter().collect()
    }

    /// Get all dominated move relationships for the given player. If a move is dominated by
    /// multiple different moves, it will contain multiple entries in the returned vector.
    ///
    /// See the documentation for [`Dominated`] for more info.
    ///
    /// # Examples
    /// ```
    /// use t4t::*;
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
    pub fn dominated_moves_for(&self, player: PlayerIndex<P>) -> Vec<Dominated<M>> {
        let mut dominated = Vec::new();

        for maybe_ted in self.possible_moves_for_player(player) {
            let ted_iter = self.possible_outcomes().include(player, maybe_ted);

            for maybe_tor in self.possible_moves_for_player(player) {
                if maybe_ted == maybe_tor {
                    continue;
                }

                let tor_iter = self.possible_outcomes().include(player, maybe_tor);

                let mut is_dominated = true;
                let mut is_strict = true;
                for (ted_outcome, tor_outcome) in ted_iter.clone().zip(tor_iter) {
                    let ted_payoff = ted_outcome.payoff();
                    let tor_payoff = tor_outcome.payoff();
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
    pub fn dominated_moves(&self) -> PerPlayer<Vec<Dominated<M>>, P> {
        PerPlayer::generate(|index| self.dominated_moves_for(index))
    }
}

impl<M: Move, U: Utility> Normal<M, U, 2> {
    /// Construct a matrix game, a two-player zero-sum game where the payoffs are defined by a
    /// single matrix of utility values.
    ///
    /// Constructed from the list of moves for each player and the matrix (in row major order) of
    /// utility values for player `P0`.
    ///
    /// # Examples
    /// ```
    /// use t4t::*;
    ///
    /// let g = Normal::matrix(
    ///     ['A', 'B', 'C'],
    ///     ['D', 'E'],
    ///     [[-3, -1],
    ///      [ 0,  2],
    ///      [ 4,  6]],
    /// );
    ///
    /// assert!(g.is_zero_sum());
    /// assert_eq!(g.payoff(Profile::new(['A', 'D'])), Payoff::from([-3, 3]));
    /// assert_eq!(g.payoff(Profile::new(['A', 'E'])), Payoff::from([-1, 1]));
    /// assert_eq!(g.payoff(Profile::new(['B', 'D'])), Payoff::from([0, 0]));
    /// assert_eq!(g.payoff(Profile::new(['B', 'E'])), Payoff::from([2, -2]));
    /// assert_eq!(g.payoff(Profile::new(['C', 'D'])), Payoff::from([4, -4]));
    /// assert_eq!(g.payoff(Profile::new(['C', 'E'])), Payoff::from([6, -6]));
    /// ```
    pub fn matrix<const ROWS: usize, const COLS: usize>(
        row_moves: [M; ROWS],
        col_moves: [M; COLS],
        row_utils: [[U; COLS]; ROWS],
    ) -> Self {
        let moves = PerPlayer::new([row_moves.to_vec(), col_moves.to_vec()]);
        let mut payoff_map = HashMap::with_capacity(ROWS * COLS);
        for (r, row_move) in row_moves.into_iter().enumerate() {
            for (c, col_move) in col_moves.into_iter().enumerate() {
                let row_util = row_utils[r][c];
                let payoff = Payoff::from([row_util, U::zero().sub(row_util)]);
                let profile = Profile::new([row_move, col_move]);
                payoff_map.insert(profile, payoff);
            }
        }
        Normal::from_payoff_map(moves, payoff_map)
    }

    /// Construct a [bimatrix game](https://en.wikipedia.org/wiki/Bimatrix_game), a two-player
    /// game where the payoffs are defined by two matrices of utilities, one for each player.
    ///
    /// Constructed from the list of moves and the matrix (in row major order) of utility values
    /// for each player.
    ///
    /// # Examples
    /// ```
    /// use t4t::*;
    ///
    /// let g = Normal::bimatrix(
    ///     ['A', 'B', 'C'],
    ///     ['D', 'E'],
    ///     [[0, 5], [4, 3], [2, 1]],
    ///     [[5, 0], [1, 2], [4, 3]],
    /// );
    ///
    /// assert_eq!(g.payoff(Profile::new(['A', 'D'])), Payoff::from([0, 5]));
    /// assert_eq!(g.payoff(Profile::new(['A', 'E'])), Payoff::from([5, 0]));
    /// assert_eq!(g.payoff(Profile::new(['B', 'D'])), Payoff::from([4, 1]));
    /// assert_eq!(g.payoff(Profile::new(['B', 'E'])), Payoff::from([3, 2]));
    /// assert_eq!(g.payoff(Profile::new(['C', 'D'])), Payoff::from([2, 4]));
    /// assert_eq!(g.payoff(Profile::new(['C', 'E'])), Payoff::from([1, 3]));
    /// ```
    pub fn bimatrix<const ROWS: usize, const COLS: usize>(
        row_moves: [M; ROWS],
        col_moves: [M; COLS],
        row_utils: [[U; COLS]; ROWS],
        col_utils: [[U; COLS]; ROWS],
    ) -> Self {
        let moves = PerPlayer::new([row_moves.to_vec(), col_moves.to_vec()]);
        let mut payoff_map = HashMap::with_capacity(ROWS * COLS);
        for (r, row_move) in row_moves.into_iter().enumerate() {
            for (c, col_move) in col_moves.into_iter().enumerate() {
                let profile = Profile::new([row_move, col_move]);
                let payoff = Payoff::from([row_utils[r][c], col_utils[r][c]]);
                payoff_map.insert(profile, payoff);
            }
        }
        Normal::from_payoff_map(moves, payoff_map)
    }

    /// Construct a [symmetric](https://en.wikipedia.org/wiki/Symmetric_game) two-player
    /// normal-form game. Constructed from a list of moves available to both players and the
    /// utility values for the `ROW` player (`P0`).
    ///
    /// # Examples
    /// ```
    /// use t4t::*;
    ///
    /// let pd = Normal::symmetric_for2(
    ///     ['C', 'D'],
    ///     [[2, 0], [3, 1]],
    /// );
    ///
    /// assert_eq!(pd.payoff(Profile::new(['C', 'C'])), Payoff::from([2, 2]));
    /// assert_eq!(pd.payoff(Profile::new(['C', 'D'])), Payoff::from([0, 3]));
    /// assert_eq!(pd.payoff(Profile::new(['D', 'C'])), Payoff::from([3, 0]));
    /// assert_eq!(pd.payoff(Profile::new(['D', 'D'])), Payoff::from([1, 1]));
    /// ```
    pub fn symmetric_for2<const SIZE: usize>(
        moves: [M; SIZE],
        row_utils: [[U; SIZE]; SIZE],
    ) -> Self {
        let all_moves = PerPlayer::init_with(moves.to_vec());
        let mut payoff_map = HashMap::with_capacity(SIZE * SIZE);
        for (r, row_move) in moves.into_iter().enumerate() {
            for (c, col_move) in moves.into_iter().enumerate() {
                let profile = Profile::new([row_move, col_move]);
                let payoff = Payoff::from([row_utils[r][c], row_utils[c][r]]);
                payoff_map.insert(profile, payoff);
            }
        }
        Normal::from_payoff_map(all_moves, payoff_map)
    }
}

impl<M: Move, U: Utility> Normal<M, U, 3> {
    /// Construct a [symmetric](https://en.wikipedia.org/wiki/Symmetric_game) three-player
    /// normal-form game. Constructed from a list of moves available to all players and the utility
    /// values for player `P0`.
    ///
    /// # Examples
    /// ```
    /// use t4t::*;
    ///
    /// let pd3 = Normal::symmetric_for3(
    ///     ['C', 'D'],
    ///     [[[4, 1], [1, 0]], [[5, 3], [3, 2]]],
    /// );
    ///
    /// assert_eq!(pd3.payoff(Profile::new(['C', 'C', 'C'])), Payoff::from([4, 4, 4]));
    /// assert_eq!(pd3.payoff(Profile::new(['C', 'C', 'D'])), Payoff::from([1, 1, 5]));
    /// assert_eq!(pd3.payoff(Profile::new(['C', 'D', 'C'])), Payoff::from([1, 5, 1]));
    /// assert_eq!(pd3.payoff(Profile::new(['C', 'D', 'D'])), Payoff::from([0, 3, 3]));
    /// assert_eq!(pd3.payoff(Profile::new(['D', 'C', 'C'])), Payoff::from([5, 1, 1]));
    /// assert_eq!(pd3.payoff(Profile::new(['D', 'C', 'D'])), Payoff::from([3, 0, 3]));
    /// assert_eq!(pd3.payoff(Profile::new(['D', 'D', 'C'])), Payoff::from([3, 3, 0]));
    /// assert_eq!(pd3.payoff(Profile::new(['D', 'D', 'D'])), Payoff::from([2, 2, 2]));
    /// ```
    pub fn symmetric_for3<const SIZE: usize>(
        moves: [M; SIZE],
        p0_utils: [[[U; SIZE]; SIZE]; SIZE],
    ) -> Self {
        let all_moves = PerPlayer::init_with(moves.to_vec());
        let mut payoff_map = HashMap::with_capacity(SIZE.pow(3));
        for (i0, m0) in moves.into_iter().enumerate() {
            for (i1, m1) in moves.into_iter().enumerate() {
                for (i2, m2) in moves.into_iter().enumerate() {
                    let u0 = p0_utils[i0][i1][i2];
                    let u1 = p0_utils[i1][i2][i0];
                    let u2 = p0_utils[i2][i0][i1];
                    let payoff = Payoff::from([u0, u1, u2]);
                    let profile = Profile::new([m0, m1, m2]);
                    payoff_map.insert(profile, payoff);
                }
            }
        }
        Normal::from_payoff_map(all_moves, payoff_map)
    }
}

impl<M: Move, U: Utility> Normal<M, U, 4> {
    /// Construct a [symmetric](https://en.wikipedia.org/wiki/Symmetric_game) four-player
    /// normal-form game. Constructed from a list of moves available to all players and the utility
    /// values for player `P0`.
    ///
    /// # Examples
    /// ```
    /// use t4t::*;
    ///
    /// let pd4 = Normal::symmetric_for4(
    ///     ['C', 'D'],
    ///     [[[[6, 2], [2, 1]], [[2, 1], [1, 0]]],
    ///      [[[7, 5], [5, 4]], [[5, 4], [4, 3]]]],
    /// );
    ///
    /// assert_eq!(pd4.payoff(Profile::new(['C', 'C', 'C', 'C'])), Payoff::from([6, 6, 6, 6]));
    /// assert_eq!(pd4.payoff(Profile::new(['C', 'C', 'C', 'D'])), Payoff::from([2, 2, 2, 7]));
    /// assert_eq!(pd4.payoff(Profile::new(['C', 'C', 'D', 'C'])), Payoff::from([2, 2, 7, 2]));
    /// assert_eq!(pd4.payoff(Profile::new(['C', 'C', 'D', 'D'])), Payoff::from([1, 1, 5, 5]));
    /// assert_eq!(pd4.payoff(Profile::new(['C', 'D', 'C', 'C'])), Payoff::from([2, 7, 2, 2]));
    /// assert_eq!(pd4.payoff(Profile::new(['C', 'D', 'C', 'D'])), Payoff::from([1, 5, 1, 5]));
    /// assert_eq!(pd4.payoff(Profile::new(['C', 'D', 'D', 'C'])), Payoff::from([1, 5, 5, 1]));
    /// assert_eq!(pd4.payoff(Profile::new(['C', 'D', 'D', 'D'])), Payoff::from([0, 4, 4, 4]));
    /// assert_eq!(pd4.payoff(Profile::new(['D', 'C', 'C', 'C'])), Payoff::from([7, 2, 2, 2]));
    /// assert_eq!(pd4.payoff(Profile::new(['D', 'C', 'C', 'D'])), Payoff::from([5, 1, 1, 5]));
    /// assert_eq!(pd4.payoff(Profile::new(['D', 'C', 'D', 'C'])), Payoff::from([5, 1, 5, 1]));
    /// assert_eq!(pd4.payoff(Profile::new(['D', 'C', 'D', 'D'])), Payoff::from([4, 0, 4, 4]));
    /// assert_eq!(pd4.payoff(Profile::new(['D', 'D', 'C', 'C'])), Payoff::from([5, 5, 1, 1]));
    /// assert_eq!(pd4.payoff(Profile::new(['D', 'D', 'C', 'D'])), Payoff::from([4, 4, 0, 4]));
    /// assert_eq!(pd4.payoff(Profile::new(['D', 'D', 'D', 'C'])), Payoff::from([4, 4, 4, 0]));
    /// assert_eq!(pd4.payoff(Profile::new(['D', 'D', 'D', 'D'])), Payoff::from([3, 3, 3, 3]));
    /// ```
    pub fn symmetric_for4<const SIZE: usize>(
        moves: [M; SIZE],
        p0_utils: [[[[U; SIZE]; SIZE]; SIZE]; SIZE],
    ) -> Self {
        let all_moves = PerPlayer::init_with(moves.to_vec());
        let mut payoff_map = HashMap::with_capacity(SIZE.pow(4));
        for (i0, m0) in moves.into_iter().enumerate() {
            for (i1, m1) in moves.into_iter().enumerate() {
                for (i2, m2) in moves.into_iter().enumerate() {
                    for (i3, m3) in moves.into_iter().enumerate() {
                        let u0 = p0_utils[i0][i1][i2][i3];
                        let u1 = p0_utils[i1][i2][i3][i0];
                        let u2 = p0_utils[i2][i3][i0][i1];
                        let u3 = p0_utils[i3][i0][i1][i2];
                        let payoff = Payoff::from([u0, u1, u2, u3]);
                        let profile = Profile::new([m0, m1, m2, m3]);
                        payoff_map.insert(profile, payoff);
                    }
                }
            }
        }
        Normal::from_payoff_map(all_moves, payoff_map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use impls::impls;
    use test_log::test;

    #[test]
    fn normal_is_send_sync() {
        assert!(impls!(Normal<(), u8, 2>: Send & Sync));
    }
}
