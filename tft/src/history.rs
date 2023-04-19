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
}
