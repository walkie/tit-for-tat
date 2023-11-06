use std::fmt::Debug;

use crate::{Move, MoveRecord, Outcome, Payoff, PlyIter, Utility};

/// For repeated games, a history of previously played games.
#[derive(Clone, Debug, PartialEq)]
pub struct History<StageOutcome, U, const P: usize> {
    outcomes: Vec<StageOutcome>,
    score: Payoff<U, P>,
}

impl<StageOutcome, U, const P: usize> Default for History<StageOutcome, U, P> {
    fn default() -> Self {
        History {
            outcomes: Vec::new(),
            score: Payoff::zeros(),
        }
    }
}

impl<StageOutcome: Outcome<M, U, P>, M: Move, U: Utility, const P: usize>
    History<StageOutcome, U, P>
{
    /// Construct a new, empty history.
    pub fn new() -> Self {
        History::default()
    }

    /// Update the history by adding a new game outcome.
    pub fn add(&mut self, outcome: StageOutcome) -> &StageOutcome {
        self.score = self.score + outcome.payoff();
        self.outcomes.push(outcome);
        self.outcomes.last().unwrap()
    }

    /// Get the current score of the game.
    pub fn score(&self) -> Payoff<U, P> {
        self.score
    }
}

impl<StageOutcome: Outcome<M, U, P>, M: Move, U: Utility, const P: usize> MoveRecord<M, P>
    for History<StageOutcome, U, P>
{
    fn to_iter(&self) -> PlyIter<M, P> {
        PlyIter::new(
            self.outcomes
                .to_iter()
                .flat_map(|outcome| outcome.record().to_iter()),
        )
    }
}

impl<StageOutcome: Outcome<M, U, P>, M: Move, U: Utility, const P: usize> Outcome<M, U, P>
    for History<StageOutcome, U, P>
{
    type Record = Self;

    fn record(&self) -> &Self {
        &self
    }

    fn payoff(&self) -> &Payoff<U, P> {
        &self.score
    }
}
