use std::fmt::Debug;

use crate::{Form, Move, Outcome, Payoff, Utility};

/// For repeated games, a history of previously played games.
#[derive(Clone, Debug, PartialEq)]
pub struct History<F: Form, M: Move, U: Utility, const P: usize> {
    outcomes: Vec<Outcome<F, M, U, P>>,
    score: Payoff<U, P>,
}

impl<F: Form, M: Move, U: Utility, const P: usize> Default for History<F, M, U, P> {
    fn default() -> Self {
        History {
            outcomes: Vec::new(),
            score: Payoff::zeros(),
        }
    }
}

impl<F: Form, M: Move, U: Utility, const P: usize> History<F, M, U, P> {
    /// Construct a new, empty history.
    pub fn new() -> Self {
        History::default()
    }

    /// Update the history by adding a new game outcome.
    pub fn add(&mut self, outcome: Outcome<F, M, U, P>) -> &Outcome<F, M, U, P> {
        self.score = self.score + outcome.payoff;
        self.outcomes.push(outcome);
        self.outcomes.last().unwrap()
    }

    /// Get the current score of the game.
    pub fn score(&self) -> Payoff<U, P> {
        self.score
    }
}
