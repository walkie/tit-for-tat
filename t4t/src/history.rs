use std::fmt::Debug;

use crate::{Kind, Move, Outcome, Payoff, Utility};

/// For repeated games, a history of previously played games.
#[derive(Clone, Debug, PartialEq)]
pub struct History<K: Kind, M: Move, U: Utility, const P: usize> {
    outcomes: Vec<Outcome<K, M, U, P>>,
    score: Payoff<U, P>,
}

impl<K: Kind, M: Move, U: Utility, const P: usize> Default for History<K, M, U, P> {
    fn default() -> Self {
        History {
            outcomes: Vec::new(),
            score: Payoff::zeros(),
        }
    }
}

impl<K: Kind, M: Move, U: Utility, const P: usize> History<K, M, U, P> {
    /// Construct a new, empty history.
    pub fn new() -> Self {
        History::default()
    }

    /// Update the history by adding a new game outcome.
    pub fn add(&mut self, outcome: Outcome<K, M, U, P>) -> &Outcome<K, M, U, P> {
        self.score = self.score + outcome.payoff;
        self.outcomes.push(outcome);
        self.outcomes.last().unwrap()
    }

    /// Get the current score of the game.
    pub fn score(&self) -> Payoff<U, P> {
        self.score
    }
}
