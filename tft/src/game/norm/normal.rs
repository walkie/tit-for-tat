use std::cmp::Ordering;
use std::collections::HashMap;
use std::iter::Iterator;
use std::rc::Rc;

use crate::core::*;
use crate::game::norm::*;

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

impl<Move: IsMove, Util: IsUtil, const N: usize> Game<N> for Normal<Move, Util, N> {
    type Move = Move;
    type Util = Util;
}

impl<Move: IsMove, Util: IsUtil, const N: usize> IsSimultaneous<N> for Normal<Move, Util, N> {
    fn payoff(&self, profile: Profile<Move, N>) -> Payoff<Util, N> {
        (*self.payoff_fn)(profile)
    }
    fn is_valid_move_for_player(&self, player: PlayerIndex<N>, the_move: Move) -> bool {
        self.moves[player].contains(&the_move)
    }
}

impl<Move: IsMove, Util: IsUtil, const N: usize> IsNormal<N> for Normal<Move, Util, N> {
    fn available_moves_for_player(&self, player: PlayerIndex<N>) -> MoveIter<Move> {
        MoveIter::new(self.moves[player].clone().into_iter())
    }
}

impl<Move: IsMove, Util: IsUtil, const N: usize> Normal<Move, Util, N> {
    /// Construct a normal-form game given the moves available to each player and a function that
    /// yields the game's payoff given a profile containing a move played by each player.
    pub fn from_payoff_fn(
        moves: PerPlayer<Vec<Move>, N>,
        payoff_fn: impl Fn(Profile<Move, N>) -> Payoff<Util, N> + 'static,
    ) -> Self {
        Normal {
            moves,
            payoff_fn: Rc::new(payoff_fn),
        }
    }

    /// Construct a normal-form game given the moves available to each player and a utility
    /// function for each player.
    pub fn from_utility_fns(
        moves: PerPlayer<Vec<Move>, N>,
        util_fns: PerPlayer<impl Fn(Move) -> Util + 'static, N>,
    ) -> Self {
        let payoff_fn = move |profile: Profile<Move, N>| {
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
    /// The resulting game will log an error and return a [zero payoff](crate::core::Payoff::zeros)
    /// for any profile not contained in the map.
    pub fn from_payoff_map(
        moves: PerPlayer<Vec<Move>, N>,
        payoff_map: HashMap<Profile<Move, N>, Payoff<Util, N>>,
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
    /// use tft::game::norm::*;
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
    /// use tft::game::norm::*;
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
    /// use tft::game::norm::*;
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
}

impl<Move: IsMove, Util: IsUtil> Normal<Move, Util, 2> {
    /// Construct a matrix game, a two-player zero-sum game where the payoffs are defined by a
    /// single matrix of utility values.
    ///
    /// Constructed from the list of moves for each player and the matrix (in row major order) of
    /// utility values for player `P0`.
    ///
    /// # Examples
    /// ```
    /// use tft::core::*;
    /// use tft::game::norm::*;
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
    /// use tft::game::norm::*;
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
