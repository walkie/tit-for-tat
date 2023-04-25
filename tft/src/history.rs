use crate::game::Game;
use crate::payoff::Payoff;

/// Record of a completed game.
#[derive(Clone, Debug, PartialEq)]
pub struct Record<G: Game<P>, const P: usize> {
    moves: G::Moves,
    payoff: Payoff<G::Utility, P>,
}

/// For repeated games, a record of previously played games.
#[derive(Clone, Debug, PartialEq)]
pub struct History<G: Game<P>, const P: usize> {
    records: Vec<Record<G, P>>,
    score: Payoff<G::Utility, P>,
}

impl<G: Game<P>, const P: usize> Record<G, P> {
    /// Construct a new record.
    pub fn new(moves: G::Moves, payoff: Payoff<G::Utility, P>) -> Self {
        Record { moves, payoff }
    }

    /// Get the moves played during this game.
    pub fn moves(&self) -> G::Moves {
        self.moves
    }

    /// Get the payoff awarded at the end of this game.
    pub fn payoff(&self) -> Payoff<G::Utility, P> {
        self.payoff
    }
}

impl<G: Game<P>, const P: usize> Default for History<G, P> {
    fn default() -> Self {
        History {
            records: Vec::new(),
            score: Payoff::zeros(),
        }
    }
}

impl<G: Game<P>, const P: usize> History<G, P> {
    /// Construct a new, empty history.
    pub fn new() -> Self {
        History::default()
    }

    /// Update the history by adding a new completed game record. Returns a reference to the newly
    /// created record.
    pub fn add(
        &mut self,
        moves: G::Moves,
        payoff: Payoff<G::Utility, P>,
    ) -> &Record<G, P> {
        self.score = self.score + payoff;
        self.records.push(Record::new(moves, payoff));
        self.records.last().unwrap()
    }

    /// Get the current score of the game.
    pub fn score(&self) -> Payoff<G::Utility, P> {
        self.score
    }
}
