use std::{fmt, hash};

use crate::{Game, MoveRecord, Outcome, Payoff, PlyIter};

/// For repeated games, a history of previously played games.
pub struct History<G: Game<P>, const P: usize> {
    pub outcomes: Vec<G::Outcome>,
    pub score: Payoff<G::Utility, P>,
}

impl<G: Game<P>, const P: usize> Clone for History<G, P> {
    fn clone(&self) -> Self {
        History {
            outcomes: self.outcomes.clone(),
            score: self.score,
        }
    }
}

impl<G: Game<P>, const P: usize> fmt::Debug for History<G, P> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("History")
            .field("outcomes", &self.outcomes)
            .field("score", &self.score)
            .finish()
    }
}

impl<G: Game<P>, const P: usize> PartialEq for History<G, P> {
    fn eq(&self, other: &Self) -> bool {
        self.outcomes == other.outcomes && self.score == other.score
    }
}

impl<G: Game<P>, const P: usize> hash::Hash for History<G, P>
where
    G::Outcome: hash::Hash,
    G::Utility: hash::Hash,
{
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.outcomes.hash(state);
        self.score.hash(state);
    }
}

impl<G: Game<P>, const P: usize> Default for History<G, P> {
    fn default() -> Self {
        History {
            outcomes: Vec::new(),
            score: Payoff::zeros(),
        }
    }
}

impl<G: Game<P>, const P: usize> History<G, P> {
    /// Construct a new, empty history.
    pub fn new() -> Self {
        History::default()
    }

    /// Update the history by adding a new game outcome.
    pub fn add(&mut self, outcome: G::Outcome) -> &G::Outcome {
        self.score = self.score + *outcome.payoff();
        self.outcomes.push(outcome);
        self.outcomes.last().unwrap()
    }

    /// Get the current score of the game.
    pub fn score(&self) -> Payoff<G::Utility, P> {
        self.score
    }
}

impl<G: Game<P>, const P: usize> MoveRecord<G::Move, P> for History<G, P> {
    fn to_iter(&self) -> PlyIter<G::Move, P> {
        PlyIter::new(
            self.outcomes
                .iter()
                .flat_map(|outcome| outcome.record().to_iter()),
        )
    }
}

impl<G: Game<P>, const P: usize> Outcome<G::Move, G::Utility, P> for History<G, P> {
    type Record = Self;

    fn record(&self) -> &Self {
        self
    }

    fn payoff(&self) -> &Payoff<G::Utility, P> {
        &self.score
    }
}
