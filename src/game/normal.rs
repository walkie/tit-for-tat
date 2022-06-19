//! Games represented in normal form. Simultaneous move games with finite move sets.

use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

use crate::core::{Payoff, PerPlayer, PlayerIndex};
use crate::game::simultaneous::Profile;

/// A simultaneous move game represented in [normal form](https://en.wikipedia.org/wiki/Normal-form_game).
///
/// The normal form representation is essentially a table of payoffs indexed by each player's move.
///
/// Since the payoff table is represented directly, normal-form games must have a finite move set
/// for each player. For games with non-finite move sets, use
/// [`Simultaneous`](crate::game::Simultaneous).
///
/// # Type variables
/// - `Move` -- The type of moves played during the game.
/// - `Util` -- The type of utility value awarded to each player in a payoff.
/// - `N` -- The number of players that play the game.
///
/// # Examples
pub struct Normal<Move, Util, const N: usize> {
    moves: PerPlayer<Vec<Move>, N>,
    profiles: Vec<PerPlayer<Move, N>>,
    payoffs: HashMap<Profile<Move, N>, Payoff<Util, N>>,
}

impl<Move, Util, const N: usize> Normal<Move, Util, N>
where
    Move: Clone + Debug + Eq + Hash,
    Util: Clone,
{
    /// Construct a normal-form game given the list of moves available to each player and a table
    /// of payoffs.
    ///
    /// The payoff table is given as a vector of payoffs in which all payoffs where player `P0`
    /// played a given move are contiguous, all payoffs where `P0` and `P1` played a given pair of
    /// moves are contiguous, and so on. In other words, the payoff table is given in
    /// ["row major" order](https://en.wikipedia.org/wiki/Matrix_representation).
    ///
    /// This operation may fail if the number of provided payoffs is fewer than the number of
    /// unique pure strategy profiles. If too many payoffs are provided, the excess payoffs will be
    /// ignored.
    ///
    /// # Examples
    pub fn new(moves: PerPlayer<Vec<Move>, N>, table: Vec<Payoff<Util, N>>) -> Option<Self> {
        let profiles: Vec<PerPlayer<Move, N>> = moves
            .clone()
            .into_iter()
            .multi_cartesian_product()
            .map(|vec| PerPlayer::new(vec.try_into().unwrap()))
            .collect();

        let mut payoffs = HashMap::with_capacity(profiles.len());
        for (profile, payoff) in profiles.iter().zip(table) {
            payoffs.insert(profile.clone(), payoff);
        }

        if payoffs.len() == profiles.len() {
            Some(Normal {
                moves,
                profiles,
                payoffs,
            })
        } else {
            None
        }
    }

    /// Construct a [bimatrix game](https://en.wikipedia.org/wiki/Bimatrix_game), a two-player
    /// normal-form game. Constructed from the list of moves and a table of utility values for each
    /// player.
    ///
    /// # Examples
    pub fn bimatrix(
        &self,
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
        Normal::new(moves, payoffs)
    }

    /// Construct a [symmetric](https://en.wikipedia.org/wiki/Symmetric_game) two-player
    /// normal-form game. Constructed from a list of moves available to each player and the payoffs
    /// for player `P0`.
    ///
    /// # Examples
    pub fn symmetric_for2(&self, moves: Vec<Move>, utils: Vec<Util>) -> Option<Normal<Move, Util, 2>> {
        let side = moves.len();
        let size = side * side;
        if utils.len() < size {
            return None;
        }

        let mut payoffs = Vec::with_capacity(size);
        for row in 0..side {
            for col in 0..side {
                let u0 = utils[row * side + col].clone();
                let u1 = utils[col * side + row].clone();
                payoffs.push(Payoff::from([u0, u1]));
            }
        }
        Normal::new(PerPlayer::new([moves.clone(), moves]), payoffs)
    }

    /// Get the available moves for the indicated player.
    pub fn available_moves(&self, player: PlayerIndex<N>) -> &[Move] {
        &self.moves[player]
    }

    /// Is this a valid move for the given player?
    pub fn is_valid_move(&self, player: PlayerIndex<N>, the_move: &Move) -> bool {
        self.moves[player].contains(the_move)
    }

    /// Is the given strategy profile valid? A profile is valid if each move is valid for the
    /// corresponding player.
    pub fn is_valid_profile(&self, profile: &Profile<Move, N>) -> bool {
        PlayerIndex::all_indexes().all(|pi| self.is_valid_move(pi, &profile[pi]))
    }

    /// A list of all pure strategy profiles for this game.
    pub fn profiles(&self) -> &[Profile<Move, N>] {
        &self.profiles
    }

    /// Get the payoff for a given strategy profile. May return `None` if the profile contains an
    /// invalid move for some player.
    pub fn payoff(&self, profile: &Profile<Move, N>) -> Option<&Payoff<Util, N>> {
        self.payoffs.get(profile)
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
