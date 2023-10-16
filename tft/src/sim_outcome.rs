use std::rc::Rc;

use crate::{Move, Payoff, PlayerIndex, Profile, ProfileIter, Record, Utility};

/// A (potential) outcome of a simultaneous game. A payoff combined with the strategy profile
/// that produced it.
///
/// For normal-form games, an outcome corresponds to a cell in the payoff table. The profile is the
/// address of the cell and the payoff is its value.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct SimOutcome<M: Move, U: Utility, const P: usize> {
    /// The profile that produced (or would produce) this outcome. Addresses a particular cell in
    /// the payoff table.
    pub profile: Profile<M, P>,
    /// The payoff associated with this outcome. The value of the corresponding cell in the payoff
    /// table.
    pub payoff: Payoff<U, P>,
}

impl<M: Move, U: Utility, const P: usize> SimOutcome<M, U, P> {
    /// Construct a new outcome.
    pub fn new(profile: Profile<M, P>, payoff: Payoff<U, P>) -> Self {
        SimOutcome { profile, payoff }
    }
}

impl<M: Move, U: Utility, const P: usize> Record<U, P> for SimOutcome<M, U, P> {
    fn payoff(&self) -> Payoff<U, P> {
        self.payoff
    }
}

/// An iterator over all possible outcomes of a [normal-form game](crate::sim::Normal).
///
/// This enumerates the cells of the payoff table in
/// [row-major order](https://en.wikipedia.org/wiki/Row-_and_column-major_order).
#[derive(Clone)]
pub struct SimOutcomeIter<'g, M: Move, U: Utility, const P: usize> {
    profile_iter: ProfileIter<'g, M, P>,
    payoff_fn: Rc<dyn Fn(Profile<M, P>) -> Payoff<U, P> + 'g>,
}

impl<'g, M: Move, U: Utility, const P: usize> SimOutcomeIter<'g, M, U, P> {
    /// Construct a new outcome iterator given an iterator over profiles and a payoff function.
    pub fn new(
        profile_iter: ProfileIter<'g, M, P>,
        payoff_fn: Rc<dyn Fn(Profile<M, P>) -> Payoff<U, P> + 'g>,
    ) -> Self {
        SimOutcomeIter {
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
    /// Multiple invocations of [`include`](SimOutcomeIter::include) and
    /// [`exclude`](SimOutcomeIter::exclude) can be chained together to add several constraints to
    /// the iterator.
    ///
    /// See the documentation for [`ProfileIter::include`](crate::sim::ProfileIter::include) for
    /// examples and more info.
    pub fn include(self, player: PlayerIndex<P>, the_move: M) -> Self {
        SimOutcomeIter {
            profile_iter: self.profile_iter.include(player, the_move),
            ..self
        }
    }

    /// Constrain the iterator to enumerate only those cells where the given player *does not* play
    /// a specific move.
    ///
    /// If the move is not a valid move for that player, then this method will have no effect.
    ///
    /// Multiple invocations of [`include`](SimOutcomeIter::include) and
    /// [`exclude`](SimOutcomeIter::exclude) can be chained together to add several constraints to
    /// the iterator.
    ///
    /// See the documentation for [`ProfileIter::exclude`](crate::sim::ProfileIter::exclude) for
    /// examples and more info.
    pub fn exclude(self, player: PlayerIndex<P>, the_move: M) -> Self {
        SimOutcomeIter {
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
    /// See the documentation for [`ProfileIter::adjacent`](crate::sim::ProfileIter::adjacent)
    /// for examples and more info.
    pub fn adjacent(self, player: PlayerIndex<P>, profile: Profile<M, P>) -> Self {
        SimOutcomeIter {
            profile_iter: self.profile_iter.adjacent(player, profile),
            ..self
        }
    }
}

impl<'g, M: Move, U: Utility, const P: usize> Iterator for SimOutcomeIter<'g, M, U, P> {
    type Item = SimOutcome<M, U, P>;
    fn next(&mut self) -> Option<Self::Item> {
        self.profile_iter.next().map(|profile| {
            let payoff = (*self.payoff_fn)(profile);
            SimOutcome::new(profile, payoff)
        })
    }
}
