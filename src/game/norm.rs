//! Games that can be represented in normal form. Simultaneous move games with finite move sets.

use std::collections::HashMap;
use std::hash::Hash;

use crate::core::{Payoff, PerPlayer, PlayerIndex};
use crate::game::traits::{Finite, Game, Simultaneous};

/// A pure strategy profile: one move played by each player.
pub type Profile<Move, const N: usize> = PerPlayer<Move, N>;

/// A finite normal-form game is one with a finite number of valid moves for each player.
pub struct Normal<Move, Utility, const N: usize> {
    moves: PerPlayer<Vec<Move>, N>,
    payoffs: HashMap<Profile<Move, N>, Payoff<Utility, N>>,
}

impl<Move, Utility, const N: usize> Game<N> for Normal<Move, Utility, N>
where
    Move: Eq,
{
    type Move = Move;
    type Utility = Utility;
    fn is_valid_move(&self, player: PlayerIndex<N>, the_move: &Move) -> bool {
        self.moves[player].contains(the_move)
    }
}

impl<Move, Utility, const N: usize> Simultaneous<N> for Normal<Move, Utility, N>
where
    Move: Eq + Hash,
    Utility: Clone,
{
    fn payoff(&self, profile: &Profile<Move, N>) -> Option<Payoff<Utility, N>> {
        self.payoffs.get(profile).cloned()
    }
}

impl<Move, Utility, const N: usize> Finite<N> for Normal<Move, Utility, N>
where
    Move: Eq,
{
    fn available_moves(&self, player: PlayerIndex<N>) -> &[Move] {
        &self.moves[player]
    }
}
