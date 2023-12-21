use std::{fmt, hash};

use crate::{
    Game, Move, Outcome, Payoff, PlayerIndex, Ply, Profile, Record, RecordIterator,
    SequentialOutcome, SimultaneousOutcome, Transcript, Utility,
};

/// For repeated games, a history of previously played games.
pub struct History<G: Game<P>, const P: usize> {
    outcomes: Vec<G::Outcome>,
    score: Payoff<G::Utility, P>,
    num_plies: usize,
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

    /// Get an iterator over the outcomes of previously played games.
    pub fn outcomes(&self) -> RecordIterator<&G::Outcome> {
        RecordIterator::new(self.outcomes.len(), self.outcomes.iter())
    }

    /// Get an iterator over the move records of previously played games.
    pub fn records(
        &self,
    ) -> RecordIterator<&<G::Outcome as Outcome<G::Move, G::Utility, P>>::Record> {
        RecordIterator::new(
            self.outcomes.len(),
            self.outcomes().map(|outcome| outcome.record()),
        )
    }

    /// Get an iterator over the payoffs of previously played games.
    pub fn payoffs(&self) -> RecordIterator<&Payoff<G::Utility, P>> {
        RecordIterator::new(
            self.outcomes.len(),
            self.outcomes().map(|outcome| outcome.payoff()),
        )
    }

    /// Get the cumulative score of all previously played games.
    pub fn score(&self) -> &Payoff<G::Utility, P> {
        &self.score
    }
}

impl<M, U, G, const P: usize> History<G, P>
where
    M: Move,
    U: Utility,
    G: Game<P, Move = M, Utility = U, Outcome = SimultaneousOutcome<M, U, P>>,
{
    /// Get an iterator over the profiles of previously played games.
    pub fn profiles(&self) -> RecordIterator<&Profile<G::Move, P>> {
        RecordIterator::new(
            self.outcomes.len(),
            self.outcomes().map(|outcome| outcome.profile()),
        )
    }

    /// Get an iterator over all of the moves played by a given player.
    pub fn moves_for_player(&self, player: PlayerIndex<P>) -> RecordIterator<G::Move> {
        RecordIterator::new(
            self.outcomes.len(),
            self.profiles().map(move |profile| profile[player]),
        )
    }
}

impl<M, U, G, const P: usize> History<G, P>
where
    M: Move,
    U: Utility,
    G: Game<P, Move = M, Utility = U, Outcome = SequentialOutcome<M, U, P>>,
{
    /// Get an iterator over the transcripts of previously played games.
    pub fn transcripts(&self) -> RecordIterator<&Transcript<G::Move, P>> {
        RecordIterator::new(
            self.outcomes.len(),
            self.outcomes().map(|outcome| outcome.transcript()),
        )
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

impl<G: Game<P>, const P: usize> Record<G::Move, P> for History<G, P> {
    fn plies(&self) -> RecordIterator<Ply<G::Move, P>> {
        RecordIterator::new(
            self.outcomes
                .iter()
                .flat_map(|outcome| outcome.record().plies())
                .collect()
                .into_iter(),
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
