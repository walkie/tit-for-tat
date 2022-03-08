use crate::game::Game;
use crate::payoff::Payoff;
use crate::per_player::{PerPlayer, PlayerIndex};

/// A pure strategy profile: one move played by each player.
pub type Profile<Move, const N: usize> = PerPlayer<Move, N>;

/// The most general form of normal-form game.
pub trait Normal<const N: usize>: Game {
    /// Is this a valid move for the given player?
    fn is_valid_move(&self, player: PlayerIndex<N>, the_move: &Self::Move) -> bool;

    /// Is this a valid strategy profile, that is, is each player's move valid for that player?
    fn is_valid_profile(&self, profile: Profile<&Self::Move, N>) -> bool {
        PlayerIndex::all_indexes().all(|pi| self.is_valid_move(pi, profile[pi]))
    }

    /// Get the payoff for a given strategy profile. May return `None` if the profile contains an
    /// invalid move for some player.
    fn payoff(played_moves: Profile<Self::Move, N>) -> Option<Payoff<Self::Utility, N>>;
}

/// A finite normal-form game is one with a finite number of valid moves for each player.
pub trait Finite<const N: usize>: Game<N> {}
