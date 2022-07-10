use std::rc::Rc;

use crate::core::{IsMove, IsUtility, Payoff, PlayerIndex, Profile, ProfileIter};

/// A (potential) outcome of a simultaneous move game. A payoff combined with the move profile that
/// produced it.
///
/// For normal-form games, an outcome corresponds to a cell in the payoff table. The profile is the
/// address of the cell and the payoff is its value.
pub struct Outcome<Move, Util, const N: usize> {
    pub profile: Profile<Move, N>,
    pub payoff: Payoff<Util, N>,
}

/// An iterator over all possible outcomes of a finite, simultaneous move game.
///
/// For normal-form games, this enumerates the cells of the payoff table in
/// [row-major order](https://en.wikipedia.org/wiki/Row-_and_column-major_order).
#[derive(Clone)]
pub struct OutcomeIter<'game, Move: Copy, Util, const N: usize> {
    profile_iter: ProfileIter<'game, Move, N>,
    payoff_fn: Rc<dyn Fn(Profile<Move, N>) -> Payoff<Util, N> + 'game>,
}

impl<'game, Move: IsMove, Util: IsUtility, const N: usize> OutcomeIter<'game, Move, Util, N> {
    /// Construct a new outcome iterator from a profile iterator and a function that returns the
    /// payoff for each profile.
    ///
    /// The payoff function can assume that it will only ever be called by valid profiles.
    pub fn new(
        profile_iter: ProfileIter<'game, Move, N>,
        payoff_fn: impl Fn(Profile<Move, N>) -> Payoff<Util, N> + 'game,
    ) -> Self {
        OutcomeIter {
            profile_iter,
            payoff_fn: Rc::new(payoff_fn),
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
    /// See the documentation for [`ProfileIter::include`](crate::core::ProfileIter::include) for
    /// examples and more info.
    pub fn include(self, player: PlayerIndex<N>, the_move: Move) -> Self {
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
    /// See the documentation for [`ProfileIter::exclude`](crate::core::ProfileIter::exclude) for
    /// examples and more info.
    pub fn exclude(self, player: PlayerIndex<N>, the_move: Move) -> Self {
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
    /// See the documentation for [`ProfileIter::adjacent`](crate::core::ProfileIter::adjacent)
    /// for examples and more info.
    pub fn adjacent(self, player: PlayerIndex<N>, profile: Profile<Move, N>) -> Self {
        OutcomeIter {
            profile_iter: self.profile_iter.adjacent(player, profile),
            ..self
        }
    }
}

impl<'game, Move: IsMove, Util: IsUtility, const N: usize> Iterator
    for OutcomeIter<'game, Move, Util, N>
{
    type Item = Outcome<Move, Util, N>;
    fn next(&mut self) -> Option<Self::Item> {
        self.profile_iter.next().map(|profile| Outcome {
            profile,
            payoff: (*self.payoff_fn)(profile),
        })
    }
}
