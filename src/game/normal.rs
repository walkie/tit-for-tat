//! Games represented in normal form. Simultaneous move games with finite move sets.

use std::collections::HashMap;
use std::hash::Hash;

use crate::core::{Payoff, PerPlayer, PlayerIndex};
use crate::game::simultaneous::Profile;

/// A simultaneous move game represented in [normal form](https://en.wikipedia.org/wiki/Normal-form_game).
///
/// The normal form representation is essentially a table in which the game's payoff is looked up
/// in a table indexed by each player's move.
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
    payoffs: HashMap<Profile<Move, N>, Payoff<Util, N>>,
}

impl<Move, Util, const N: usize> Normal<Move, Util, N>
where
    Move: Eq + Hash,
{
    pub fn available_moves(&self, player: PlayerIndex<N>) -> &[Move] {
        &self.moves[player]
    }

    pub fn is_valid_move(&self, player: PlayerIndex<N>, the_move: &Move) -> bool {
        self.moves[player].contains(the_move)
    }

    pub fn payoff(&self, profile: &Profile<Move, N>) -> Option<&Payoff<Util, N>> {
        self.payoffs.get(profile)
    }
}
