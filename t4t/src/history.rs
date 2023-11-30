use std::{fmt, hash};

use crate::{
    Game, Move, MoveRecord, Outcome, Payoff, PlayerIndex, PlyIter, Profile, SequentialOutcome,
    SimultaneousOutcome, Transcript, Utility,
};

pub trait HistoryIterator<T>: ExactSizeIterator + DoubleEndedIterator<Item = T> {}

impl<I, T> HistoryIterator<T> for I where I: ExactSizeIterator + DoubleEndedIterator<Item = T> {}

/// For repeated games, a history of previously played games.
pub struct History<G: Game<P>, const P: usize> {
    outcomes: Vec<G::Outcome>,
    score: Payoff<G::Utility, P>,
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
    pub fn outcomes(&self) -> impl HistoryIterator<&G::Outcome> + '_ {
        self.outcomes.iter()
    }

    /// Get an iterator over the move records of previously played games.
    pub fn records(
        &self,
    ) -> impl HistoryIterator<&<G::Outcome as Outcome<G::Move, G::Utility, P>>::Record> + '_ {
        self.outcomes().map(|outcome| outcome.record())
    }

    /// Get an iterator over the payoffs of previously played games.
    pub fn payoffs(&self) -> impl HistoryIterator<&Payoff<G::Utility, P>> + '_ {
        self.outcomes().map(|outcome| outcome.payoff())
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
    pub fn profiles(&self) -> impl HistoryIterator<&Profile<G::Move, P>> + '_ {
        self.outcomes().map(|outcome| outcome.profile())
    }

    /// Get an iterator over all of the moves played by a given player.
    pub fn moves_for_player(&self, player: PlayerIndex<P>) -> impl HistoryIterator<G::Move> + '_ {
        self.profiles().map(move |profile| profile[player])
    }
}

impl<M, U, G, const P: usize> History<G, P>
where
    M: Move,
    U: Utility,
    G: Game<P, Move = M, Utility = U, Outcome = SequentialOutcome<M, U, P>>,
{
    /// Get an iterator over the transcripts of previously played games.
    pub fn transcripts(&self) -> impl HistoryIterator<&Transcript<G::Move, P>> + '_ {
        self.outcomes().map(|outcome| outcome.transcript())
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
