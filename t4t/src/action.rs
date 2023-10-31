use crate::{Distribution, Move, Payoff, PlayerIndex, Profile, State, Utility};

pub trait Next<T, S, M, U, const P: usize>:
    FnOnce(S, T) -> (S, Action<S, M, U, P>) + 'static
{
}

impl<F, T, S, M, U, const P: usize> Next<T, S, M, U, P> for F where
    F: FnOnce(S, T) -> (S, Action<S, M, U, P>) + 'static
{
}

/// An action indicates the next step to be performed while playing a game.
pub enum Action<S, M, U, const P: usize> {
    /// A turn taken by one or more players simultaneously.
    ///
    /// The argument vector should never be empty and should probably not contain the same player
    /// more than once.
    Turns {
        players: Vec<PlayerIndex<P>>,
        next: Box<dyn Next<Vec<M>, S, M, U, P>>,
    },

    /// A move of chance according to the given distribution.
    Chance {
        distribution: Distribution<M>,
        next: Box<dyn Next<M, S, M, U, P>>,
    },

    /// A payoff that terminates the game.
    Payoff {
        payoff: Payoff<U, P>,
        next: Option<Box<dyn Next<(), S, M, U, P>>>,
    },
}

impl<S: State, M: Move, U: Utility, const P: usize> Action<S, M, U, P> {
    /// Construct a turn action where all players move simultaneously.
    pub fn simultaneous(next: impl Next<Profile<M, P>, S, M, U, P>) -> Self {
        Action::Turns {
            players: PlayerIndex::all_indexes().into_iter().collect(),
            next: Box::new(next),
        }
    }

    /// Construct a turn action for a single player.
    pub fn turn(player: PlayerIndex<P>, next: impl Next<M, S, M, U, P>) -> Self {
        let turns_next = Box::new(move |state, moves: Vec<M>| {
            assert_eq!(moves.len(), 1);
            next(state, moves[0])
        });

        Action::Turns {
            players: vec![player],
            next: turns_next,
        }
    }

    pub fn turns(players: Vec<PlayerIndex<P>>, next: impl Next<Vec<M>, S, M, U, P>) -> Self {
        Action::Turns {
            players,
            next: Box::new(next),
        }
    }

    pub fn chance(distribution: Distribution<M>, next: impl Next<M, S, M, U, P>) -> Self {
        Action::Chance {
            distribution,
            next: Box::new(next),
        }
    }

    pub fn payoff(distribution: Distribution<M>, next: impl Next<(), S, M, U, P>) -> Self {
        Action::Chance {
            distribution,
            next: Box::new(next),
        }
    }
}
