use crate::moves::IsMove;
use crate::payoff::{IsUtility, Payoff};
use crate::seq::transcript::{PlayedMove, Transcript};

/// A (potential) outcome of a sequential game. A payoff combined with the transcript of moves that
/// produced it.
///
/// For extensive-form games, an outcome corresponds to a path through the game tree.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Outcome<Move, Util, const N: usize> {
    /// The transcript of moves that produced (or would produce) this outcome. Defines a path
    /// through the game tree.
    pub transcript: Transcript<Move, N>,
    /// The payoff associated with this outcome. The value at the leaf of the game tree.
    pub payoff: Payoff<Util, N>,
}

impl<Move: IsMove, Util: IsUtility, const N: usize> Outcome<Move, Util, N> {
    /// Construct a new outcome.
    pub fn new(transcript: Transcript<Move, N>, payoff: Payoff<Util, N>) -> Self {
        Outcome { transcript, payoff }
    }

    /// Construct a new outcome from the outcome of a simultaneous game.
    pub fn from_sim_outcome(sim_outcome: crate::sim::Outcome<Move, Util, N>) -> Self {
        let moves = sim_outcome
            .profile
            .map_with_index(|p, m| PlayedMove::player(p, m))
            .into_iter()
            .collect();
        Outcome::new(Transcript::from_played_moves(moves), sim_outcome.payoff)
    }
}
