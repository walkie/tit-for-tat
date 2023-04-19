use crate::moves::IsMove;
use crate::outcome::Outcome;
use crate::payoff::{IsUtility, Payoff};
use crate::profile::Profile;
use crate::transcript::Transcript;

/// Record of a completed game.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Record<Move, Util, const N: usize> {
    moves: Moves<Move, N>,
    payoff: Payoff<Util, N>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
enum Moves<Move, const N: usize> {
    Sequential(Transcript<Move, N>),
    Simultaneous(Profile<Move, N>),
}

impl<Move: IsMove, Util: IsUtility, const N: usize> Record<Move, Util, N> {
    /// Construct a record of a completed sequential game.
    pub fn sequential(transcript: Transcript<Move, N>, payoff: Payoff<Util, N>) -> Self {
        Record {
            moves: Moves::Sequential(transcript),
            payoff,
        }
    }

    /// Construct a record of a completed simultaneous game.
    pub fn simultaneous(profile: Profile<Move, N>, payoff: Payoff<Util, N>) -> Self {
        Record {
            moves: Moves::Simultaneous(profile),
            payoff,
        }
    }

    /// Convert a sequential game outcome to a completed game record.
    pub fn from_outcome(outcome: Outcome<Move, Util, N>) -> Self {
        Record::simultaneous(outcome.profile, outcome.payoff)
    }
}
