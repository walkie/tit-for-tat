use std::rc::Rc;

use crate::game::{Move, Utility};
use crate::payoff::Payoff;
use crate::per_player::PlayerIndex;
use crate::profile::{Profile, ProfileIter};

/// A (potential) outcome of a simultaneous game. A payoff combined with the strategy profile
/// that produced it.
///
/// For normal-form games, an outcome corresponds to a cell in the payoff table. The profile is the
/// address of the cell and the payoff is its value.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Outcome<M, U, const N: usize> {
    /// The profile that produced (or would produce) this outcome. Addresses a particular cell in
    /// the payoff table.
    pub profile: Profile<M, N>,
    /// The payoff associated with this outcome. The value of the corresponding cell in the payoff
    /// table.
    pub payoff: Payoff<U, N>,
}

/// An iterator over all possible outcomes of a [normal-form](crate::Normal) game.
///
/// This enumerates the cells of the payoff
/// table in [row-major order](https://en.wikipedia.org/wiki/Row-_and_column-major_order).
#[derive(Clone)]
pub struct OutcomeIter<'g, M: Copy, U, const N: usize> {
    profile_iter: ProfileIter<'g, M, N>,
    payoff_fn: Rc<dyn Fn(Profile<M, N>) -> Payoff<U, N> + 'g>,
}

impl<M: Move, U: Utility, const N: usize> Outcome<M, U, N> {
    /// Construct a new outcome.
    pub fn new(profile: Profile<M, N>, payoff: Payoff<U, N>) -> Self {
        Outcome { profile, payoff }
    }
}

impl<'g, M: Move, U: Utility, const N: usize> OutcomeIter<'g, M, U, N> {
    /// Construct a new outcome iterator given an iterator over profiles and a payoff function.
    pub fn new(
        profile_iter: ProfileIter<'g, M, N>,
        payoff_fn: Rc<dyn Fn(Profile<M, N>) -> Payoff<U, N> + 'g>,
    ) -> Self {
        OutcomeIter {
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
    /// Multiple invocations of [`include`](OutcomeIter::include) and
    /// [`exclude`](OutcomeIter::exclude) can be chained together to add several constraints to
    /// the iterator.
    ///
    /// See the documentation for [`ProfileIter::include`](crate::ProfileIter::include) for
    /// examples and more info.
    pub fn include(self, player: PlayerIndex<N>, the_move: M) -> Self {
        OutcomeIter {
            profile_iter: self.profile_iter.include(player, the_move),
            ..self
        }
    }

    /// Constrain the iterator to enumerate only those cells where the given player *does not* play
    /// a specific move.
    ///
    /// If the move is not a valid move for that player, then this method will have no effect.
    ///
    /// Multiple invocations of [`include`](OutcomeIter::include) and
    /// [`exclude`](OutcomeIter::exclude) can be chained together to add several constraints to
    /// the iterator.
    ///
    /// See the documentation for [`ProfileIter::exclude`](crate::ProfileIter::exclude) for
    /// examples and more info.
    pub fn exclude(self, player: PlayerIndex<N>, the_move: M) -> Self {
        OutcomeIter {
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
    /// See the documentation for [`ProfileIter::adjacent`](crate::ProfileIter::adjacent)
    /// for examples and more info.
    pub fn adjacent(self, player: PlayerIndex<N>, profile: Profile<M, N>) -> Self {
        OutcomeIter {
            profile_iter: self.profile_iter.adjacent(player, profile),
            ..self
        }
    }
}

impl<'g, M: Move, U: Utility, const N: usize> Iterator for OutcomeIter<'g, M, U, N> {
    type Item = Outcome<M, U, N>;
    fn next(&mut self) -> Option<Self::Item> {
        self.profile_iter.next().map(|profile| {
            let payoff = (*self.payoff_fn)(profile);
            Outcome::new(profile, payoff)
        })
    }
}
