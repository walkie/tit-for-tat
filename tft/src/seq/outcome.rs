use crate::history::Record;
use crate::moves::Move;
use crate::payoff::{Payoff, Utility};
use crate::seq::transcript::{Played, Transcript};

/// A (potential) outcome of a sequential game. A payoff combined with the transcript of moves that
/// produced it.
///
/// For extensive-form games, an outcome corresponds to a path through the game tree.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Outcome<M, U, const P: usize> {
    /// The transcript of moves that produced (or would produce) this outcome. Defines a path
    /// through the game tree.
    pub transcript: Transcript<M, P>,
    /// The payoff associated with this outcome. The value at the leaf of the game tree.
    pub payoff: Payoff<U, P>,
}

impl<M: Move, U: Utility, const P: usize> Outcome<M, U, P> {
    /// Construct a new outcome.
    pub fn new(transcript: Transcript<M, P>, payoff: Payoff<U, P>) -> Self {
        Outcome { transcript, payoff }
    }

    /// Construct a new outcome from the outcome of a simultaneous game.
    pub fn from_sim_outcome(sim_outcome: crate::sim::Outcome<M, U, P>) -> Self {
        let moves = sim_outcome
            .profile
            .map_with_index(|p, m| Played::player(p, m))
            .into_iter()
            .collect();
        Outcome::new(Transcript::from_played_moves(moves), sim_outcome.payoff)
    }
}

impl<M, U: Utility, const P: usize> Record<U, P> for Outcome<M, U, P> {
    fn payoff(&self) -> Payoff<U, P> {
        self.payoff
    }
}
