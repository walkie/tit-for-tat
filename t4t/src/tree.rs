use std::sync::Arc;

use crate::{Distribution, ErrorKind, Move, PlayerIndex, Profile};

/// The outgoing edges of a node in a game tree, represented as a function.
///
/// The function yields the next node in the tree, given the current game state and the result of
/// this node's action.
///
/// This trait is effectively a type synonym for the function type it extends. A blanket
/// implementation covers all possible instances, so it should not be implemented directly.
pub trait NextNode<'g, T, S, M, O, const P: usize>:
    FnOnce(Arc<S>, T) -> Result<Node<'g, S, M, O, P>, ErrorKind<M, P>> + 'g
{
}

impl<'g, F, T, S, M, O, const P: usize> NextNode<'g, T, S, M, O, P> for F where
    F: FnOnce(Arc<S>, T) -> Result<Node<'g, S, M, O, P>, ErrorKind<M, P>> + 'g
{
}

/// A node in a game tree. The current game state and an [action][Action] to perform.
///
/// Subsequent nodes, if applicable, are reachable via the action's `next` function.
pub struct Node<'g, S, M, O, const P: usize> {
    /// The game state at this node.
    pub state: Arc<S>,
    /// The action to take at this node.
    pub action: Action<'g, S, M, O, P>,
}

/// The game action to perform at a given node in the game tree.
pub enum Action<'g, S, M, O, const P: usize> {
    /// One or more players play a move simultaneously.
    Turn {
        /// The players to move simultaneously.
        to_move: Vec<PlayerIndex<P>>,
        /// Compute the next node from the moves played by the players.
        next: Box<dyn NextNode<'g, Vec<M>, S, M, O, P>>,
    },

    /// Make a move of chance according to the given distribution.
    Chance {
        /// The distribution to draw a move from.
        distribution: Distribution<M>,
        /// Compute the next node from the move drawn from the distribution.
        next: Box<dyn NextNode<'g, M, S, M, O, P>>,
    },

    /// End a game and return the outcome, which includes the game's payoff.
    End {
        /// The final outcome of the game.
        outcome: O,
    },
}

impl<'g, S, M: Move, O, const P: usize> Action<'g, S, M, O, P> {
    /// Construct an action where a single player must make a move and the next node is computed
    /// from the move they choose.
    pub fn player(to_move: PlayerIndex<P>, next: impl NextNode<'g, M, S, M, O, P>) -> Self {
        Action::players(vec![to_move], move |state, moves| {
            assert_eq!(moves.len(), 1);
            next(state, moves[0])
        })
    }

    /// Construct an action where several players must make a move simultaneously and the next node
    /// is computed from the moves they choose.
    pub fn players(
        to_move: Vec<PlayerIndex<P>>,
        next: impl NextNode<'g, Vec<M>, S, M, O, P>,
    ) -> Self {
        Action::Turn {
            to_move,
            next: Box::new(next),
        }
    }

    /// Construct an action where all players must make a move simultaneously and the next node is
    /// computed from the moves they choose.
    pub fn all_players(next: impl NextNode<'g, Profile<M, P>, S, M, O, P>) -> Self {
        Action::players(PlayerIndex::all().collect(), move |state, moves| {
            assert_eq!(moves.len(), P);
            next(state, Profile::new(moves.try_into().unwrap()))
        })
    }

    /// Construct an action where a move is selected from a distribution and the next node is
    /// computed from the selected move.
    pub fn chance(distribution: Distribution<M>, next: impl NextNode<'g, M, S, M, O, P>) -> Self {
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

impl<'g, S, M: Move, O, const P: usize> Node<'g, S, M, O, P> {
    /// Construct a new game node with the given state and action.
    pub fn new(state: Arc<S>, action: Action<'g, S, M, O, P>) -> Self {
        Node { state, action }
    }

    /// Construct a game node where a single player must make a move and the next node is computed
    /// from the move they choose.
    pub fn player(
        state: Arc<S>,
        player: PlayerIndex<P>,
        next: impl NextNode<'g, M, S, M, O, P>,
    ) -> Self {
        Node::new(state, Action::player(player, next))
    }

    /// Construct a game node where several players must make a move simultaneously and the next
    /// node is computed from the moves they choose.
    pub fn players(
        state: Arc<S>,
        players: Vec<PlayerIndex<P>>,
        next: impl NextNode<'g, Vec<M>, S, M, O, P>,
    ) -> Self {
        Node::new(state, Action::players(players, next))
    }

    /// Construct a game node where all players must make a move simultaneously and the next node
    /// is computed from the moves they choose.
    pub fn all_players(state: Arc<S>, next: impl NextNode<'g, Profile<M, P>, S, M, O, P>) -> Self {
        Node::new(state, Action::all_players(next))
    }

    /// Construct a game node where a move is selected from a distribution and the next node is
    /// computed from the selected move.
    pub fn chance(
        state: Arc<S>,
        distribution: Distribution<M>,
        next: impl NextNode<'g, M, S, M, O, P>,
    ) -> Self {
        Node::new(state, Action::chance(distribution, next))
    }

    /// Construct a game node ending the game with the given outcome.
    pub fn end(state: Arc<S>, outcome: O) -> Self {
        Node::new(state, Action::end(outcome))
    }
}
