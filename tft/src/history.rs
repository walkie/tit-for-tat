use std::fmt::Debug;

use crate::{Payoff, Utility};

/// A record of a completed game iteration.
pub trait Record<U: Utility, const P: usize>: Clone + Debug + PartialEq {
    /// The payoff awarded at the end of the game.
    fn payoff(&self) -> Payoff<U, P>;
}

/// For repeated games, a history of previously played games.
#[derive(Clone, Debug, PartialEq)]
pub struct History<U: Utility, R: Record<U, P>, const P: usize> {
    records: Vec<R>,
    score: Payoff<U, P>,
}

impl<U: Utility, R: Record<U, P>, const P: usize> Default for History<U, R, P> {
    fn default() -> Self {
        History {
            records: Vec::new(),
            score: Payoff::zeros(),
        }
    }
}

impl<U: Utility, R: Record<U, P>, const P: usize> History<U, R, P> {
    /// Construct a new, empty history.
    pub fn new() -> Self {
        History::default()
    }

    /// Update the history by adding a new completed game record.
    pub fn add(&mut self, record: R) -> &R {
        self.score = self.score + record.payoff();
        self.records.push(record);
        self.records.last().unwrap()
    }

    /// Get the current score of the game.
    pub fn score(&self) -> Payoff<U, P> {
        self.score
    }
}

impl<U: Utility, R: Record<U, P>, const P: usize> Record<U, P> for History<U, R, P> {
    fn payoff(&self) -> Payoff<U, P> {
        self.score()
    }
}
