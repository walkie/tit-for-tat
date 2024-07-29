use std::marker::PhantomData;

use crate::{
    Move, Outcome, Past, Payoff, PlayerIndex, Plies, Profile, Record, SequentialOutcome,
    SimultaneousOutcome, Summary, Transcript, Utility,
};

/// For repeated games, a history of previously played games.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct History<M: Move, U: Utility, O: Outcome<M, U, P>, const P: usize> {
    outcomes: im::Vector<O>,
    score: Payoff<U, P>,
    summary: Summary<P>,
    move_type: PhantomData<M>,
}

impl<M: Move, U: Utility, O: Outcome<M, U, P>, const P: usize> History<M, U, O, P> {
    /// Construct a new, empty history.
    pub fn empty() -> Self {
        History::default()
    }

    /// Update the history by adding a new game outcome. Returns a reference to the newly added
    /// outcome.
    pub fn add(&mut self, outcome: O) -> &O {
        self.score = self.score + *outcome.payoff();
        self.summary = self.summary + outcome.record().summary();
        self.outcomes.push_back(outcome);
        self.outcomes.back().unwrap()
    }

    /// Get an iterator over the outcomes of previously played games.
    pub fn outcomes(&self) -> Past<&O> {
        Past::from_iter(self.outcomes.len(), self.outcomes.iter())
    }

    /// Get an iterator over the move records of previously played games.
    pub fn records(&self) -> Past<&<O as Outcome<M, U, P>>::Record> {
        Past::from_iter(
            self.outcomes.len(),
            self.outcomes().map(|outcome| outcome.record()),
        )
    }

    /// Get an iterator over the payoffs of previously played games.
    pub fn payoffs(&self) -> Past<&Payoff<U, P>> {
        Past::from_iter(
            self.outcomes.len(),
            self.outcomes().map(|outcome| outcome.payoff()),
        )
    }

    /// Get the cumulative score of all previously played games.
    pub fn score(&self) -> &Payoff<U, P> {
        &self.score
    }
}

impl<M: Move, U: Utility, const P: usize> History<M, U, SimultaneousOutcome<M, U, P>, P> {
    /// Get an iterator over the profiles of previously played games.
    pub fn profiles(&self) -> Past<&Profile<M, P>> {
        Past::from_iter(
            self.outcomes.len(),
            self.outcomes().map(|outcome| outcome.profile()),
        )
    }

    /// Get an iterator over all moves played by a given player.
    pub fn moves_for_player(&self, player: PlayerIndex<P>) -> Past<M> {
        Past::from_iter(
            self.outcomes.len(),
            self.profiles().map(move |profile| profile[player]),
        )
    }
}

impl<M: Move, U: Utility, const P: usize> History<M, U, SequentialOutcome<M, U, P>, P> {
    /// Get an iterator over the transcripts of previously played games.
    pub fn transcripts(&self) -> Past<&Transcript<M, P>> {
        Past::from_iter(
            self.outcomes.len(),
            self.outcomes().map(|outcome| outcome.transcript()),
        )
    }
}

impl<M: Move, U: Utility, O: Outcome<M, U, P>, const P: usize> Default for History<M, U, O, P> {
    fn default() -> Self {
        History {
            outcomes: im::Vector::new(),
            score: Payoff::zeros(),
            summary: Summary::empty(),
            move_type: PhantomData,
        }
    }
}

impl<M: Move, U: Utility, O: Outcome<M, U, P>, const P: usize> Record<M, P>
    for History<M, U, O, P>
{
    fn plies(&self) -> Plies<M, P> {
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

impl<M: Move, U: Utility, O: Outcome<M, U, P>, const P: usize> Outcome<M, U, P>
    for History<M, U, O, P>
{
    type Record = Self;

    fn record(&self) -> &Self {
        self
    }

    fn payoff(&self) -> &Payoff<U, P> {
        &self.score
    }
}
