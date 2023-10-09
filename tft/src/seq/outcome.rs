use crate::seq::Transcript;
use crate::{Move, Payoff, Record, Utility};

/// A (potential) outcome of a sequential game. A payoff combined with the transcript of moves that
/// produced it.
///
/// For extensive-form games, an outcome corresponds to a path through the game tree.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Outcome<M: Move, U: Utility, const P: usize> {
    /// The transcript of moves that produced (or would produce) this outcome. Defines a path
    /// through the game tree.
    pub transcript: Transcript<M, P>,
    /// The payoff associated with this outcome. The value at the leaf of the game tree.
    pub payoff: Payoff<U, P>,
}

impl<M: Move, U: Utility, const P: usize> Record<U, P> for Outcome<M, U, P> {
    fn payoff(&self) -> Payoff<U, P> {
        self.payoff
    }
}

impl<M: Move, U: Utility, const P: usize> Outcome<M, U, P> {
    /// Construct a new outcome.
    pub fn new(transcript: Transcript<M, P>, payoff: Payoff<U, P>) -> Self {
        Outcome { transcript, payoff }
    }
}
