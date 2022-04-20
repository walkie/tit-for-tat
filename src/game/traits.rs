use crate::core::{Payoff, PerPlayer, PlayerIndex};

/// A pure strategy profile for a one-move simultaneous game. One move played by each player.
pub type Profile<Move, const N: usize> = PerPlayer<Move, N>;

pub trait Game<const N: usize> {
    /// The type of moves played during the game.
    type Move;
    /// The type of utility value awarded to each player in a payoff.
    type Utility;

    /// Is this a valid move for the given player?
    fn is_valid_move(&self, player: PlayerIndex<N>, the_move: &Self::Move) -> bool;
}

pub trait Simultaneous<const N: usize>: Game<N> {
    /// Get the payoff for a given strategy profile. May return `None` if the profile contains an
    /// invalid move for some player.
    fn payoff(&self, profile: &Profile<Self::Move, N>) -> Option<Payoff<Self::Utility, N>>;

    /// Is the given strategy profile valid? A profile is valid if each move is valid for the
    /// corresponding player.
    fn is_valid_profile(&self, profile: &Profile<Self::Move, N>) -> bool {
        PlayerIndex::all_indexes().all(|pi| self.is_valid_move(pi, &profile[pi]))
    }

    /// The payoff method should yield a payoff for every valid profile. This function checks
    /// whether this property holds for a given profile and is intended for use in tests.
    fn law_valid_profile_yields_payoff(&self, profile: &Profile<Self::Move, N>) -> bool {
        if self.is_valid_profile(profile) {
            self.payoff(profile).is_some()
        } else {
            true // ok to return a meaningless payoff for an invalid profile
        }
    }
}

/// A finite game has a finite set of available moves at each decision point.
pub trait Finite<const N: usize>: Game<N> {
    /// Get the set of moves available to a player.
    fn available_moves(&self, player: PlayerIndex<N>) -> &[Self::Move];
}
