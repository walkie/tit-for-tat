//! Simultaneous games.

use crate::core::{Payoff, PerPlayer, PlayerIndex};

/// A pure strategy profile: one move played by each player.
pub type Profile<Move, const NP: usize> = PerPlayer<Move, NP>;

/// The most general form of simultaneous move game. This representation is best used for games
/// with non-finite domains of moves.
pub struct Simultaneous<Move, Util, const NP: usize> {
    payoff_fn: Box<dyn Fn(&Profile<Move, NP>) -> Option<Payoff<Util, NP>>>,
    move_fn: Box<dyn Fn(PlayerIndex<NP>, Move) -> bool>,
}

impl<Move, Util, const NP: usize> Simultaneous<Move, Util, NP> {
    pub fn new(
        payoff_fn: impl Fn(&Profile<Move, NP>) -> Option<Payoff<Util, NP>> + 'static,
        move_fn: impl Fn(PlayerIndex<NP>, Move) -> bool + 'static,
    ) -> Self {
        Simultaneous {
            payoff_fn: Box::new(payoff_fn),
            move_fn: Box::new(move_fn),
        }
    }

    /// Get the payoff for a given strategy profile. May return `None` if the profile contains an
    /// invalid move for some player.
    pub fn payoff(&self, profile: &Profile<Move, NP>) -> Option<Payoff<Util, NP>> {
        (*self.payoff_fn)(profile)
    }

    /// Is this a valid move for the given player?
    pub fn is_valid_move(&self, player: PlayerIndex<NP>, the_move: Move) -> bool {
        (*self.move_fn)(player, the_move)
    }
}
