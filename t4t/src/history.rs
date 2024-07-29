use std::{fmt, hash};

use crate::{
    Move, Outcome, Past, Payoff, Playable, PlayerIndex, Plies, Profile, Record, SequentialOutcome,
    SimultaneousOutcome, Summary, Transcript, Utility,
};

/// For repeated games, a history of previously played games.
pub struct History<G: Playable<P>, const P: usize> {
    outcomes: im::Vector<G::Outcome>,
    score: Payoff<G::Utility, P>,
    summary: Summary<P>,
}

impl<G: Playable<P>, const P: usize> History<G, P> {
    /// Construct a new, empty history.
    pub fn empty() -> Self {
        History::default()
    }

    /// Update the history by adding a new game outcome. Returns a reference to the newly added
    /// outcome.
    pub fn add(&mut self, outcome: G::Outcome) -> &G::Outcome {
        self.score = self.score + *outcome.payoff();
        self.summary = self.summary + outcome.record().summary();
        self.outcomes.push_back(outcome);
        self.outcomes.back().unwrap()
    }

    /// Get an iterator over the outcomes of previously played games.
    pub fn outcomes(&self) -> Past<&G::Outcome> {
        Past::from_iter(self.outcomes.len(), self.outcomes.iter())
    }

    /// Get an iterator over the move records of previously played games.
    pub fn records(&self) -> Past<&<G::Outcome as Outcome<G::Move, G::Utility, P>>::Record> {
        Past::from_iter(
            self.outcomes.len(),
            self.outcomes().map(|outcome| outcome.record()),
        )
    }

    /// Get an iterator over the payoffs of previously played games.
    pub fn payoffs(&self) -> Past<&Payoff<G::Utility, P>> {
        Past::from_iter(
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
    G: Playable<P, Move = M, Utility = U, Outcome = SimultaneousOutcome<M, U, P>>,
{
    /// Get an iterator over the profiles of previously played games.
    pub fn profiles(&self) -> Past<&Profile<G::Move, P>> {
        Past::from_iter(
            self.outcomes.len(),
            self.outcomes().map(|outcome| outcome.profile()),
        )
    }

    /// Get an iterator over all moves played by a given player.
    pub fn moves_for_player(&self, player: PlayerIndex<P>) -> Past<G::Move> {
        Past::from_iter(
            self.outcomes.len(),
            self.profiles().map(move |profile| profile[player]),
        )
    }
}

impl<M, U, G, const P: usize> History<G, P>
where
    M: Move,
    U: Utility,
    G: Playable<P, Move = M, Utility = U, Outcome = SequentialOutcome<M, U, P>>,
{
    /// Get an iterator over the transcripts of previously played games.
    pub fn transcripts(&self) -> Past<&Transcript<G::Move, P>> {
        Past::from_iter(
            self.outcomes.len(),
            self.outcomes().map(|outcome| outcome.transcript()),
        )
    }
}

impl<G: Playable<P>, const P: usize> Default for History<G, P> {
    fn default() -> Self {
        History {
            outcomes: im::Vector::new(),
            score: Payoff::zeros(),
            summary: Summary::empty(),
        }
    }
}

impl<G: Playable<P>, const P: usize> Record<G::Move, P> for History<G, P> {
    fn plies(&self) -> Plies<G::Move, P> {
        Past::from_iter(
            self.outcomes.len(),
            self.outcomes
                .iter()
                .flat_map(|outcome| outcome.record().plies()),
        )
    }

    fn summary(&self) -> Summary<P> {
        self.summary
    }
}

impl<G: Playable<P>, const P: usize> Outcome<G::Move, G::Utility, P> for History<G, P> {
    type Record = Self;

    fn record(&self) -> &Self {
        self
    }

    fn payoff(&self) -> &Payoff<G::Utility, P> {
        &self.score
    }
}

impl<G: Playable<P>, const P: usize> Clone for History<G, P> {
    fn clone(&self) -> Self {
        History {
            outcomes: self.outcomes.clone(),
            score: self.score,
            summary: self.summary,
        }
    }
}

impl<G: Playable<P>, const P: usize> fmt::Debug for History<G, P> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("History")
            .field("outcomes", &self.outcomes)
            .field("score", &self.score)
            .finish()
    }
}

impl<G: Playable<P>, const P: usize> PartialEq for History<G, P> {
    fn eq(&self, other: &Self) -> bool {
        self.outcomes == other.outcomes && self.score == other.score
    }
}

impl<G: Playable<P>, const P: usize> hash::Hash for History<G, P>
where
    G::Outcome: hash::Hash,
    G::Utility: hash::Hash,
{
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.outcomes.hash(state);
        self.score.hash(state);
    }
}
