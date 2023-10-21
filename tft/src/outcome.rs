use std::rc::Rc;

use crate::{Kind, Move, Payoff, PlayerIndex, Profile, ProfileIter, Sim, Utility};

/// A (potential) outcome of a game. A payoff combined with a record of the moves that produced it.
///
/// For normal-form games, an outcome corresponds to a cell in the payoff table. The profile is the
/// address of the cell and the payoff is its value.
///
/// For extensive-form games, an outcome corresponds to a path through the game tree.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Outcome<K: Kind, M: Move, U: Utility, const P: usize> {
    /// A record of the moves that produced (or would produce) this outcome.
    ///
    /// For simultaneous games, this will be a [profile](crate::Profile) containing one move for
    /// each player. For sequential games, it will be a [transcript](crate::Transcript) of all moves
    /// played in the game.
    pub record: K::Record<M, P>,

    /// The payoff associated with this outcome.
    pub payoff: Payoff<U, P>,
}

impl<K: Kind, M: Move, U: Utility, const P: usize> Outcome<K, M, U, P> {
    pub fn new(record: K::Record<M, P>, payoff: Payoff<U, P>) -> Self {
        Outcome { record, payoff }
    }

    pub fn payoff(&self) -> &Payoff<U, P> {
        &self.payoff
    }
}

/// An iterator over all possible outcomes of a [normal-form game](crate::prelude::sim::Normal).
///
/// This enumerates the cells of the payoff table in
/// [row-major order](https://en.wikipedia.org/wiki/Row-_and_column-major_order).
#[derive(Clone)]
pub struct OutcomeIter<'g, M: Move, U: Utility, const P: usize> {
    profile_iter: ProfileIter<'g, M, P>,
    payoff_fn: Rc<dyn Fn(Profile<M, P>) -> Payoff<U, P> + 'g>,
}

impl<'g, M: Move, U: Utility, const P: usize> OutcomeIter<'g, M, U, P> {
    /// Construct a new outcome iterator given an iterator over profiles and a payoff function.
    pub fn new(
        profile_iter: ProfileIter<'g, M, P>,
        payoff_fn: Rc<dyn Fn(Profile<M, P>) -> Payoff<U, P> + 'g>,
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
    /// See the documentation for [`ProfileIter::include`](crate::prelude::sim::ProfileIter::include) for
    /// examples and more info.
    pub fn include(self, player: PlayerIndex<P>, the_move: M) -> Self {
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
    /// See the documentation for [`ProfileIter::exclude`](crate::prelude::sim::ProfileIter::exclude) for
    /// examples and more info.
    pub fn exclude(self, player: PlayerIndex<P>, the_move: M) -> Self {
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
    /// See the documentation for [`ProfileIter::adjacent`](crate::prelude::sim::ProfileIter::adjacent)
    /// for examples and more info.
    pub fn adjacent(self, player: PlayerIndex<P>, profile: Profile<M, P>) -> Self {
        OutcomeIter {
            profile_iter: self.profile_iter.adjacent(player, profile),
            ..self
        }
    }
}

impl<'g, M: Move, U: Utility, const P: usize> Iterator for OutcomeIter<'g, M, U, P> {
    type Item = Outcome<Sim, M, U, P>;
    fn next(&mut self) -> Option<Self::Item> {
        self.profile_iter.next().map(|profile| {
            let payoff = (*self.payoff_fn)(profile);
            Outcome::new(profile, payoff)
        })
    }
}
