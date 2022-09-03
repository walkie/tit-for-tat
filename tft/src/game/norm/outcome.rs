use std::rc::Rc;

use crate::core::{IsMove, IsUtil, Payoff, PlayerIndex};
use crate::game::norm::IsNormal;
use crate::game::sim::{Profile, ProfileIter};

/// A (potential) outcome of a simultaneous move game. A payoff combined with the strategy profile
/// that produced it.
///
/// For normal-form games, an outcome corresponds to a cell in the payoff table. The profile is the
/// address of the cell and the payoff is its value.
pub struct Outcome<Move, Util, const N: usize> {
    /// The profile that produced (or would produce) this outcome. Addresses a particular cell in
    /// the payoff table.
    pub profile: Profile<Move, N>,
    /// The payoff associated with this outcome. The value of the corresponding cell in the payoff
    /// table.
    pub payoff: Payoff<Util, N>,
}

/// An iterator over all possible outcomes of a [normal-form](crate::game::sim::IsNormal) game.
///
/// This enumerates the cells of the payoff
/// table in [row-major order](https://en.wikipedia.org/wiki/Row-_and_column-major_order).
#[derive(Clone)]
pub struct OutcomeIter<'g, Move: Copy, Util, const N: usize> {
    profile_iter: ProfileIter<'g, Move, N>,
    payoff_fn: Rc<dyn Fn(Profile<Move, N>) -> Payoff<Util, N> + 'g>,
}

impl<'g, Move: IsMove, Util: IsUtil, const N: usize> OutcomeIter<'g, Move, Util, N> {
    /// Construct a new outcome iterator for the given finite simultaneous-move game.
    pub fn for_game(game: &'g (impl IsNormal<N, Move = Move, Util = Util> + ?Sized)) -> Self {
        OutcomeIter {
            profile_iter: game.profiles(),
            payoff_fn: Rc::new(|profile| game.payoff(profile)),
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
    /// See the documentation for [`ProfileIter::include`](crate::norm::ProfileIter::include) for
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
    /// See the documentation for [`ProfileIter::exclude`](crate::norm::ProfileIter::exclude) for
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
    /// See the documentation for [`ProfileIter::adjacent`](crate::norm::ProfileIter::adjacent)
    /// for examples and more info.
    pub fn adjacent(self, player: PlayerIndex<N>, profile: Profile<Move, N>) -> Self {
        OutcomeIter {
            profile_iter: self.profile_iter.adjacent(player, profile),
            ..self
        }
    }
}

impl<'g, Move: IsMove, Util: IsUtil, const N: usize> Iterator for OutcomeIter<'g, Move, Util, N> {
    type Item = Outcome<Move, Util, N>;
    fn next(&mut self) -> Option<Self::Item> {
        self.profile_iter.next().map(|profile| {
            let payoff = (*self.payoff_fn)(profile);
            Outcome { profile, payoff }
        })
    }
}
