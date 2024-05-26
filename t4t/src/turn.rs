use std::rc::Rc;

use crate::{Distribution, ErrorKind, Move, PlayerIndex, Profile};

/// A function that yields the next turn in the game, given the current state and the result of the
/// previous turn.
///
/// This trait is effectively a type synonym for the function type it extends. A blanket
/// implementation covers all possible instances, so it should not be implemented directly.
pub trait NextTurn<'g, T, S, M, O, const P: usize>:
    FnOnce(Rc<S>, T) -> Result<Turn<'g, S, M, O, P>, ErrorKind<M, P>> + 'g
{
}

impl<'g, F, T, S, M, O, const P: usize> NextTurn<'g, T, S, M, O, P> for F where
    F: FnOnce(Rc<S>, T) -> Result<Turn<'g, S, M, O, P>, ErrorKind<M, P>> + 'g
{
}

/// A description of one step in a game's execution.
///
/// Subsequent steps, if applicable, are reachable via the action's `next` method.
pub struct Turn<'g, S, M, O, const P: usize> {
    /// The game state at this turn.
    pub state: Rc<S>,
    /// The action to take at this turn.
    pub action: Action<'g, S, M, O, P>,
}

/// The next action to perform while playing a game.
pub enum Action<'g, S, M, O, const P: usize> {
    /// One or more players play a move simultaneously.
    Players {
        /// The players to move simultaneously.
        to_move: Vec<PlayerIndex<P>>,
        /// Compute the next turn from the moves played by the players.
        next: Box<dyn NextTurn<'g, Vec<M>, S, M, O, P>>,
    },

    /// A move of chance according to the given distribution.
    Chance {
        /// The distribution to draw a move from.
        distribution: Distribution<M>,
        /// Compute the next turn from the move drawn from the distribution.
        next: Box<dyn NextTurn<'g, M, S, M, O, P>>,
    },

    /// Award a payoff to the players and terminate the game.
    End {
        /// The final outcome of the game.
        outcome: O,
    },
}

impl<'g, S, M: Move, O, const P: usize> Action<'g, S, M, O, P> {
    /// Construct an action where a single player must make a move and the next turn is computed
    /// from the move they choose.
    pub fn player(to_move: PlayerIndex<P>, next: impl NextTurn<'g, M, S, M, O, P>) -> Self {
        Action::players(vec![to_move], move |state, moves| {
            assert_eq!(moves.len(), 1);
            next(state, moves[0])
        })
    }

    /// Construct an action where several players must make a move simultaneously and the next turn
    /// is computed from the moves they choose.
    pub fn players(
        to_move: Vec<PlayerIndex<P>>,
        next: impl NextTurn<'g, Vec<M>, S, M, O, P>,
    ) -> Self {
        Action::Players {
            to_move,
            next: Box::new(next),
        }
    }

    /// Construct an action where all players must make a move simultaneously and the next turn is
    /// computed from the moves they choose.
    pub fn all_players(next: impl NextTurn<'g, Profile<M, P>, S, M, O, P>) -> Self {
        Action::players(PlayerIndex::all().collect(), move |state, moves| {
            assert_eq!(moves.len(), P);
            next(state, Profile::new(moves.try_into().unwrap()))
        })
    }

    /// Construct an action where a move is selected from a distribution and the next turn is
    /// computed from the selected move.
    pub fn chance(distribution: Distribution<M>, next: impl NextTurn<'g, M, S, M, O, P>) -> Self {
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

impl<'g, S, M: Move, O, const P: usize> Turn<'g, S, M, O, P> {
    /// Construct a new turn with the given state and action.
    pub fn new(state: Rc<S>, action: Action<'g, S, M, O, P>) -> Self {
        Turn { state, action }
    }

    /// Construct a turn where a single player must make a move and the next turn is computed
    /// from the move they choose.
    pub fn player(
        state: Rc<S>,
        player: PlayerIndex<P>,
        next: impl NextTurn<'g, M, S, M, O, P>,
    ) -> Self {
        Turn::new(state, Action::player(player, next))
    }

    /// Construct a turn where several players must make a move simultaneously and the next turn
    /// is computed from the moves they choose.
    pub fn players(
        state: Rc<S>,
        players: Vec<PlayerIndex<P>>,
        next: impl NextTurn<'g, Vec<M>, S, M, O, P>,
    ) -> Self {
        Turn::new(state, Action::players(players, next))
    }

    /// Construct a turn where all players must make a move simultaneously and the next turn is
    /// computed from the moves they choose.
    pub fn all_players(state: Rc<S>, next: impl NextTurn<'g, Profile<M, P>, S, M, O, P>) -> Self {
        Turn::new(state, Action::all_players(next))
    }

    /// Construct a turn where a move is selected from a distribution and the next turn is
    /// computed from the selected move.
    pub fn chance(
        state: Rc<S>,
        distribution: Distribution<M>,
        next: impl NextTurn<'g, M, S, M, O, P>,
    ) -> Self {
        Turn::new(state, Action::chance(distribution, next))
    }

    /// Construct a turn ending the game with the given outcome.
    pub fn end(state: Rc<S>, outcome: O) -> Self {
        Turn::new(state, Action::end(outcome))
    }
}
