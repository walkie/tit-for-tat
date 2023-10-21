use std::fmt::Debug;

use crate::{Outcome, Payoff, Utility};

/// For repeated games, a history of previously played games.
#[derive(Clone, Debug, PartialEq)]
pub struct History<R, U: Utility, const P: usize> {
    outcomes: Vec<Outcome<R, U, P>>,
    score: Payoff<U, P>,
}

impl<R, U: Utility, const P: usize> Default for History<R, U, P> {
    fn default() -> Self {
        History {
            outcomes: Vec::new(),
            score: Payoff::zeros(),
        }
    }
}

impl<R, U: Utility, const P: usize> History<R, U, P> {
    /// Construct a new, empty history.
    pub fn new() -> Self {
        History::default()
    }

    /// Update the history by adding a new game outcome.
    pub fn add(&mut self, outcome: Outcome<R, U, P>) -> &Outcome<R, U, P> {
        self.score = self.score + outcome.payoff;
        self.outcomes.push(outcome);
        self.outcomes.last().unwrap()
    }

    /// Get the current score of the game.
    pub fn score(&self) -> Payoff<U, P> {
        self.score
    }
}
