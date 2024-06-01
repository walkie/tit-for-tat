use std::sync::Arc;

use crate::{Distribution, ErrorKind, Move, PlayerIndex, Profile};

/// A step-by-step executable description of how a game is played.
///
/// This type is an alias used for the first step in a game's rules.
pub type Rules<'g, S, M, O, const P: usize> = Step<'g, S, M, O, P>;

/// A function that yields the next step in a game's rules, given the current state and the
/// result of the previous step.
///
/// This trait is effectively a type synonym for the function type it extends. A blanket
/// implementation covers all possible instances, so it should not be implemented directly.
pub trait NextStep<'g, T, S, M, O, const P: usize>:
    FnOnce(Arc<S>, T) -> Result<Step<'g, S, M, O, P>, ErrorKind<M, P>> + 'g
{
}

impl<'g, F, T, S, M, O, const P: usize> NextStep<'g, T, S, M, O, P> for F where
    F: FnOnce(Arc<S>, T) -> Result<Step<'g, S, M, O, P>, ErrorKind<M, P>> + 'g
{
}

/// One step in a game's [rules][crate::Rules].
///
/// Subsequent steps, if applicable, are reachable via the action's `next` method.
pub struct Step<'g, S, M, O, const P: usize> {
    /// The game state at this step.
    pub state: Arc<S>,
    /// The action to take at this step.
    pub action: Action<'g, S, M, O, P>,
}

/// The action to perform at a given step in a game's rules.
pub enum Action<'g, S, M, O, const P: usize> {
    /// One or more players to play a move simultaneously.
    Turn {
        /// The players to move simultaneously.
        to_move: Vec<PlayerIndex<P>>,
        /// Compute the next step from the moves played by the players.
        next: Box<dyn NextStep<'g, Vec<M>, S, M, O, P>>,
    },

    /// A move of chance according to the given distribution.
    Chance {
        /// The distribution to draw a move from.
        distribution: Distribution<M>,
        /// Compute the next step from the move drawn from the distribution.
        next: Box<dyn NextStep<'g, M, S, M, O, P>>,
    },

    /// Award a payoff to the players and end the game.
    End {
        /// The final outcome of the game.
        outcome: O,
    },
}

impl<'g, S, M: Move, O, const P: usize> Action<'g, S, M, O, P> {
    /// Construct an action where a single player must make a move and the next step is computed
    /// from the move they choose.
    pub fn player(to_move: PlayerIndex<P>, next: impl NextStep<'g, M, S, M, O, P>) -> Self {
        Action::players(vec![to_move], move |state, moves| {
            assert_eq!(moves.len(), 1);
            next(state, moves[0])
        })
    }

    /// Construct an action where several players must make a move simultaneously and the next step
    /// is computed from the moves they choose.
    pub fn players(
        to_move: Vec<PlayerIndex<P>>,
        next: impl NextStep<'g, Vec<M>, S, M, O, P>,
    ) -> Self {
        Action::Turn {
            to_move,
            next: Box::new(next),
        }
    }

    /// Construct an action where all players must make a move simultaneously and the next step is
    /// computed from the moves they choose.
    pub fn all_players(next: impl NextStep<'g, Profile<M, P>, S, M, O, P>) -> Self {
        Action::players(PlayerIndex::all().collect(), move |state, moves| {
            assert_eq!(moves.len(), P);
            next(state, Profile::new(moves.try_into().unwrap()))
        })
    }

    /// Construct an action where a move is selected from a distribution and the next step is
    /// computed from the selected move.
    pub fn chance(distribution: Distribution<M>, next: impl NextStep<'g, M, S, M, O, P>) -> Self {
        Action::Chance {
            distribution,
            next: Box::new(next),
        }
    }

    /// Construct an action ending the game with the given outcome.
    pub fn end(outcome: O) -> Self {
        Action::End { outcome }
    }
}

impl<'g, S, M: Move, O, const P: usize> Step<'g, S, M, O, P> {
    /// Construct a new rules step with the given state and action.
    pub fn new(state: Arc<S>, action: Action<'g, S, M, O, P>) -> Self {
        Step { state, action }
    }

    /// Construct a rules step where a single player must make a move and the next step is computed
    /// from the move they choose.
    pub fn player(
        state: Arc<S>,
        player: PlayerIndex<P>,
        next: impl NextStep<'g, M, S, M, O, P>,
    ) -> Self {
        Step::new(state, Action::player(player, next))
    }

    /// Construct a rules step where several players must make a move simultaneously and the next
    /// step is computed from the moves they choose.
    pub fn players(
        state: Arc<S>,
        players: Vec<PlayerIndex<P>>,
        next: impl NextStep<'g, Vec<M>, S, M, O, P>,
    ) -> Self {
        Step::new(state, Action::players(players, next))
    }

    /// Construct a rules step where all players must make a move simultaneously and the next step
    /// is computed from the moves they choose.
    pub fn all_players(state: Arc<S>, next: impl NextStep<'g, Profile<M, P>, S, M, O, P>) -> Self {
        Step::new(state, Action::all_players(next))
    }

    /// Construct a rules step where a move is selected from a distribution and the next step is
    /// computed from the selected move.
    pub fn chance(
        state: Arc<S>,
        distribution: Distribution<M>,
        next: impl NextStep<'g, M, S, M, O, P>,
    ) -> Self {
        Step::new(state, Action::chance(distribution, next))
    }

    /// Construct a rules step ending the game with the given outcome.
    pub fn end(state: Arc<S>, outcome: O) -> Self {
        Step::new(state, Action::end(outcome))
    }
}
