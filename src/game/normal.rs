//! Games represented in normal form. Simultaneous move games with finite move sets.

use std::collections::HashMap;
use std::hash::Hash;

use crate::core::{Payoff, PerPlayer, PlayerIndex};
use crate::game::simultaneous::Profile;

/// A simultaneous move game represented in [normal form](https://en.wikipedia.org/wiki/Normal-form_game),
/// that is, as a table in which the payoff is looked up in a table indexed by each player's move.
///
/// Since the payoff table is represented directly, normal-form games must have a finite move set
/// for each player. For games with non-finite move sets, use
/// [`Simultaneous`](crate::game::Simultaneous).
///
/// # Type variables
/// - `Move` -- The type of moves played during the game.
/// - `Utility` -- The type of utility value awarded to each player in a payoff.
/// - `NUM_PLAYERS` -- The number of players that play the game.
///
/// # Examples
pub struct Normal<Move, Utility, const NUM_PLAYERS: usize> {
    moves: PerPlayer<Vec<Move>, NUM_PLAYERS>,
    payoffs: HashMap<Profile<Move, NUM_PLAYERS>, Payoff<Utility, NUM_PLAYERS>>,
}

impl<Move, Utility, const NUM_PLAYERS: usize> Normal<Move, Utility, NUM_PLAYERS>
where
    Move: Eq + Hash,
{
    pub fn available_moves(&self, player: PlayerIndex<NUM_PLAYERS>) -> &[Move] {
        &self.moves[player]
    }

    pub fn is_valid_move(&self, player: PlayerIndex<NUM_PLAYERS>, the_move: &Move) -> bool {
        self.moves[player].contains(the_move)
    }

    pub fn payoff(
        &self,
        profile: &Profile<Move, NUM_PLAYERS>,
    ) -> Option<&Payoff<Utility, NUM_PLAYERS>> {
        self.payoffs.get(profile)
    }
}
