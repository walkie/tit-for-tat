use std::fmt::Debug;
use std::sync::Arc;

use crate::{Move, Payoff, PlayerIndex, PossibleProfiles, Profile, Record, Transcript, Utility};

/// A (potential) outcome of a game. A payoff combined with a record of the moves that produced it.
///
/// The outcomes for some built-in game types are:
/// - [`SimultaneousOutcome`] for [simultaneous](crate::Simultaneous) and
///   [normal-form](crate::Normal) games.
/// - [`SequentialOutcome`] for sequential games.
/// - [`History`](crate::History) for [repeated](crate::Repeated) games.
pub trait Outcome<M: Move, U: Utility, const P: usize>:
    Clone + Debug + PartialEq + Send + Sync
{
    /// A type for capturing the record of moves that produced (or would produce) this outcome.
    ///
    /// For simultaneous games, this will be a [profile](Profile) containing one move for each
    /// player. For sequential games, it will be a [transcript](Transcript) of all moves played in
    /// the game.
    type Record: Record<M, P>;

    /// A record of the moves that produced this outcome.
    fn record(&self) -> &Self::Record;

    /// The payoff associated with this outcome.
    fn payoff(&self) -> &Payoff<U, P>;
}

/// A (potential) outcome of a simultaneous game.
///
/// The profile of moves played by each player and the resulting payoff.
///
/// For normal-form games, an outcome corresponds to a cell in the payoff table. The profile is the
/// address of the cell, the payoff is its value.
#[derive(Clone, Debug, PartialEq, Hash)]
pub struct SimultaneousOutcome<M: Move, U: Utility, const P: usize> {
    profile: Profile<M, P>,
    payoff: Payoff<U, P>,
}

impl<M: Move, U: Utility, const P: usize> SimultaneousOutcome<M, U, P> {
    /// Construct a new simultaneous game outcome.
    pub fn new(profile: Profile<M, P>, payoff: Payoff<U, P>) -> Self {
        SimultaneousOutcome { profile, payoff }
    }

    /// The move profile associated with this outcome.
    pub fn profile(&self) -> &Profile<M, P> {
        &self.profile
    }
}

impl<M: Move, U: Utility, const P: usize> Outcome<M, U, P> for SimultaneousOutcome<M, U, P> {
    type Record = Profile<M, P>;

    fn record(&self) -> &Profile<M, P> {
        &self.profile
    }

    fn payoff(&self) -> &Payoff<U, P> {
        &self.payoff
    }
}

/// A (potential) outcome of a sequential game.
///
/// A transcript of moves played by all players and the resulting payoff.
///
/// For extensive-form games, an outcome corresponds to a path through the game tree.
#[derive(Clone, Debug, PartialEq, Hash)]
pub struct SequentialOutcome<M: Move, U: Utility, const P: usize> {
    transcript: Transcript<M, P>,
    payoff: Payoff<U, P>,
}

impl<M: Move, U: Utility, const P: usize> SequentialOutcome<M, U, P> {
    /// Construct a new sequential game outcome.
    pub fn new(transcript: Transcript<M, P>, payoff: Payoff<U, P>) -> Self {
        SequentialOutcome { transcript, payoff }
    }

    /// The move transcript associated with this outcome.
    pub fn transcript(&self) -> &Transcript<M, P> {
        &self.transcript
    }
}

impl<M: Move, U: Utility, const P: usize> Outcome<M, U, P> for SequentialOutcome<M, U, P> {
    type Record = Transcript<M, P>;

    fn record(&self) -> &Transcript<M, P> {
        &self.transcript
    }

    fn payoff(&self) -> &Payoff<U, P> {
        &self.payoff
    }
}

/// An iterator over all possible outcomes of a [normal-form game](crate::Normal).
///
/// This enumerates the cells of the payoff table in
/// [row-major order](https://en.wikipedia.org/wiki/Row-_and_column-major_order).
#[derive(Clone)]
pub struct PossibleOutcomes<'g, M: Move, U: Utility, const P: usize> {
    profile_iter: PossibleProfiles<'g, M, P>,
    payoff_fn: Arc<dyn Fn(Profile<M, P>) -> Payoff<U, P> + Send + Sync + 'g>,
}

impl<'g, M: Move, U: Utility, const P: usize> PossibleOutcomes<'g, M, U, P> {
    /// Construct a new outcome iterator given an iterator over profiles and a payoff function.
    pub fn new(
        profile_iter: PossibleProfiles<'g, M, P>,
        payoff_fn: Arc<dyn Fn(Profile<M, P>) -> Payoff<U, P> + Send + Sync + 'g>,
    ) -> Self {
        PossibleOutcomes {
            profile_iter,
            payoff_fn,
        }
    }

    /// Constrain the iterator to enumerate only those cells where the given player plays a
    /// specific move.
    ///
    /// If the move is not a valid move for that player, then the resulting iterator will not
    /// generate any profiles.
    ///
    /// Multiple invocations of [`include`](PossibleOutcomes::include) and
    /// [`exclude`](PossibleOutcomes::exclude) can be chained together to add several constraints to
    /// the iterator.
    ///
    /// See the documentation for [`ProfileIter::include`](PossibleProfiles::include) for
    /// examples and more info.
    pub fn include(self, player: PlayerIndex<P>, the_move: M) -> Self {
        PossibleOutcomes {
            profile_iter: self.profile_iter.include(player, the_move),
            ..self
        }
    }

    /// Constrain the iterator to enumerate only those cells where the given player *does not* play
    /// a specific move.
    ///
    /// If the move is not a valid move for that player, then this method will have no effect.
    ///
    /// Multiple invocations of [`include`](PossibleOutcomes::include) and
    /// [`exclude`](PossibleOutcomes::exclude) can be chained together to add several constraints to
    /// the iterator.
    ///
    /// See the documentation for [`ProfileIter::exclude`](PossibleProfiles::exclude) for
    /// examples and more info.
    pub fn exclude(self, player: PlayerIndex<P>, the_move: M) -> Self {
        PossibleOutcomes {
            profile_iter: self.profile_iter.exclude(player, the_move),
            ..self
        }
    }

    /// Constrain the iterator to generate only cells that correspond to "adjacent" profiles of the
    /// given profile for a given player.
    ///
    /// An adjacent profile is one where the given player plays a different move, but all other
    /// players play the move specified in the profile.
    ///
    /// Note that this doesn't correspond to adjacency in the payoff table, but rather an entire
    /// row or column, minus the provided profile.
    ///
    /// See the documentation for [`ProfileIter::adjacent`](PossibleProfiles::adjacent)
    /// for examples and more info.
    pub fn adjacent(self, player: PlayerIndex<P>, profile: Profile<M, P>) -> Self {
        PossibleOutcomes {
            profile_iter: self.profile_iter.adjacent(player, profile),
            ..self
        }
    }
}

impl<'g, M: Move, U: Utility, const P: usize> Iterator for PossibleOutcomes<'g, M, U, P> {
    type Item = SimultaneousOutcome<M, U, P>;
    fn next(&mut self) -> Option<Self::Item> {
        self.profile_iter.next().map(|profile| {
            let payoff = (*self.payoff_fn)(profile);
            SimultaneousOutcome::new(profile, payoff)
        })
    }
}

impl<M: Move, U: Utility, O: Outcome<M, U, P>, const P: usize> Outcome<M, U, P> for Arc<O> {
    type Record = O::Record;

    fn record(&self) -> &Self::Record {
        self.as_ref().record()
    }

    fn payoff(&self) -> &Payoff<U, P> {
        self.as_ref().payoff()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use impls::impls;
    use test_log::test;

    #[test]
    fn possible_outcomes_is_send_sync() {
        assert!(impls!(PossibleOutcomes<'_, (), u8, 2>: Send & Sync));
    }
}
