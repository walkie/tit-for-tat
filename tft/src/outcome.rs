use std::rc::Rc;

use crate::moves::IsMove;
use crate::payoff::{IsUtil, Payoff};
use crate::per_player::PlayerIndex;
use crate::profile::{Profile, ProfileIter};
use crate::transcript::{PlayedMove, Transcript};

/// A (potential) outcome of a sequential game. A payoff combined with the transcript of moves that
/// produced it.
///
/// For extensive-form games, an outcome corresponds to a path through the game tree.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct SeqOutcome<Move, Util, const N: usize> {
    /// The transcript of moves that produced (or would produce) this outcome. Defines a path
    /// through the game tree.
    pub transcript: Transcript<Move, N>,
    /// The payoff associated with this outcome. The value at the leaf of the game tree.
    pub payoff: Payoff<Util, N>,
}

/// A (potential) outcome of a simultaneous game. A payoff combined with the strategy profile
/// that produced it.
///
/// For normal-form games, an outcome corresponds to a cell in the payoff table. The profile is the
/// address of the cell and the payoff is its value.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct SimOutcome<Move, Util, const N: usize> {
    /// The profile that produced (or would produce) this outcome. Addresses a particular cell in
    /// the payoff table.
    pub profile: Profile<Move, N>,
    /// The payoff associated with this outcome. The value of the corresponding cell in the payoff
    /// table.
    pub payoff: Payoff<Util, N>,
}

/// An iterator over all possible outcomes of a [normal-form](crate::Normal) game.
///
/// This enumerates the cells of the payoff
/// table in [row-major order](https://en.wikipedia.org/wiki/Row-_and_column-major_order).
#[derive(Clone)]
pub struct SimOutcomeIter<'g, Move: Copy, Util, const N: usize> {
    profile_iter: ProfileIter<'g, Move, N>,
    payoff_fn: Rc<dyn Fn(Profile<Move, N>) -> Payoff<Util, N> + 'g>,
}

impl<Move: IsMove, Util: IsUtil, const N: usize> SeqOutcome<Move, Util, N> {
    /// Construct a new outcome.
    pub fn new(transcript: Transcript<Move, N>, payoff: Payoff<Util, N>) -> Self {
        SeqOutcome { transcript, payoff }
    }

    /// Convert a sequential outcome to a simultaneous outcome, if possible.
    ///
    /// Returns `None` if the transcript does not contain exactly one move per player.
    pub fn to_sim_outcome(&self) -> Option<SimOutcome<Move, Util, N>> {
        self.transcript
            .to_profile()
            .map(|profile| SimOutcome::new(profile, self.payoff))
    }
}

impl<Move: IsMove, Util: IsUtil, const N: usize> SimOutcome<Move, Util, N> {
    /// Construct a new outcome.
    pub fn new(profile: Profile<Move, N>, payoff: Payoff<Util, N>) -> Self {
        SimOutcome { profile, payoff }
    }

    /// Convert a simultaneous outcome to a sequential outcome.
    pub fn to_seq_outcome(&self) -> SeqOutcome<Move, Util, N> {
        let moves = self
            .profile
            .map_with_index(|p, m| PlayedMove::player(p, m))
            .into_iter()
            .collect();
        SeqOutcome::new(Transcript::from_played_moves(moves), self.payoff)
    }
}

impl<'g, Move: IsMove, Util: IsUtil, const N: usize> SimOutcomeIter<'g, Move, Util, N> {
    /// Construct a new outcome iterator given an iterator over profiles and a payoff function.
    pub fn new(
        profile_iter: ProfileIter<'g, Move, N>,
        payoff_fn: Rc<dyn Fn(Profile<Move, N>) -> Payoff<Util, N> + 'g>,
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
    /// See the documentation for [`ProfileIter::include`](crate::norm::ProfileIter::include) for
    /// examples and more info.
    pub fn include(self, player: PlayerIndex<N>, the_move: Move) -> Self {
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
    /// See the documentation for [`ProfileIter::exclude`](crate::norm::ProfileIter::exclude) for
    /// examples and more info.
    pub fn exclude(self, player: PlayerIndex<N>, the_move: Move) -> Self {
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
    /// See the documentation for [`ProfileIter::adjacent`](crate::norm::ProfileIter::adjacent)
    /// for examples and more info.
    pub fn adjacent(self, player: PlayerIndex<N>, profile: Profile<Move, N>) -> Self {
        SimOutcomeIter {
            profile_iter: self.profile_iter.adjacent(player, profile),
            ..self
        }
    }
}

impl<'g, Move: IsMove, Util: IsUtil, const N: usize> Iterator
    for SimOutcomeIter<'g, Move, Util, N>
{
    type Item = SimOutcome<Move, Util, N>;
    fn next(&mut self) -> Option<Self::Item> {
        self.profile_iter.next().map(|profile| {
            let payoff = (*self.payoff_fn)(profile);
            SimOutcome::new(profile, payoff)
        })
    }
}
