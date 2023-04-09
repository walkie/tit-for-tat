use crate::outcome::Outcome;
use crate::payoff::Payoff;
use crate::per_player::PlayerIndex;

/// A move played during a game.
pub struct PlayedMove<Move, const N: usize> {
    /// The player that played the move, or `None` if it was a move of chance.
    pub player: Option<PlayerIndex<N>>,
    /// The move that was played.
    pub the_move: Move,
}

/// A transcript of a completed game.
pub struct Transcript<Move, Util, const N: usize> {
    /// The sequence of moves played in the game.
    moves: Vec<PlayedMove<Move, N>>,
    /// The payoff achieved at the end of the game.
    payoff: Payoff<Util, N>,
}

// impl<Move: IsMove, Util: IsUtil, const N: usize> Transcript<Move, Util, N> {
//     pub fn from_outcome(outcome: Outcome<Move, Util, N>) -> Self {
//     }
// }
