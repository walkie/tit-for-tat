use crate::moves::IsMove;
use crate::payoff::{IsUtility, Payoff};
use crate::record::Record;

/// For repeated games, a record of previously played games.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct History<Move, Util, const N: usize> {
    records: Vec<Record<Move, Util, N>>,
    score: Payoff<Util, N>,
}

impl<Move, Util: IsUtility, const N: usize> Default for History<Move, Util, N> {
    fn default() -> Self {
        History {
            records: Vec::new(),
            score: Payoff::zeros(),
        }
    }
}

impl<Move: IsMove, Util: IsUtility, const N: usize> History<Move, Util, N> {
    /// Construct a new, empty history.
    pub fn new() -> Self {
        History::default()
    }

    /// Update the history by adding a new completed game record.
    pub fn add_record(&mut self, record: Record<Move, Util, N>) {
        self.score = self.score + record.payoff();
        self.records.push(record);
    }

    /// Get the current score of the game.
    pub fn score(&self) -> Payoff<Util, N> {
        self.score
    }
}
