use std::marker::PhantomData;
use std::sync::Arc;

use crate::{
    Distribution, ErrorKind, Game, Move, Outcome, Playable, PlayerIndex, Profile, State, Utility,
};

/// The outgoing edges of a node in a game tree, represented as a function.
///
/// The function yields the next node in the tree, given the current game state and the result of
/// this node's action.
///
/// This trait is effectively a type synonym for the function type it extends. A blanket
/// implementation covers all possible instances, so it should not be implemented directly.
pub trait NextGameTree<T, S, M, U, O, const P: usize>:
    Fn(S, T) -> Result<GameTree<S, M, U, O, P>, ErrorKind<M, P>> + Send + Sync + 'static
{
}

impl<F, T, S, M, U, O, const P: usize> NextGameTree<T, S, M, U, O, P> for F where
    F: Fn(S, T) -> Result<GameTree<S, M, U, O, P>, ErrorKind<M, P>> + Send + Sync + 'static
{
}

/// A node in a game tree. The current game state and an [action](Action) to perform.
///
/// Subsequent nodes, if applicable, are reachable via the action's `next` function.
#[derive(Clone)]
pub struct GameTree<S, M, U, O, const P: usize> {
    /// The game state at this node.
    pub state: S,
    /// The action to take at this node.
    pub action: Action<S, M, U, O, P>,
}

/// The game action to perform at a given node in the game tree.
#[derive(Clone)]
pub enum Action<S, M, U, O, const P: usize> {
    /// One or more players play a move simultaneously.
    Turns {
        /// The players to move simultaneously.
        to_move: Vec<PlayerIndex<P>>,
        /// Compute the next node from the moves played by the players.
        next: Arc<dyn NextGameTree<Vec<M>, S, M, U, O, P>>,
    },

    /// Make a move of chance according to the given distribution.
    Chance {
        /// The distribution to draw a move from.
        distribution: Distribution<M>,
        /// Compute the next node from the move drawn from the distribution.
        next: Arc<dyn NextGameTree<M, S, M, U, O, P>>,
    },

    /// End a game and return the outcome, which includes the game's payoff.
    End {
        /// The final outcome of the game.
        outcome: O,
        /// Phantom data to specify the utility value type.
        utility_type: PhantomData<U>,
    },
}

impl<S, M: Move, U, O, const P: usize> GameTree<S, M, U, O, P> {
    /// Construct a new game node with the given state and action.
    pub fn new(state: S, action: Action<S, M, U, O, P>) -> Self {
        GameTree { state, action }
    }

    /// Construct a game node where a single player must make a move and the next node is computed
    /// from the move they choose.
    pub fn player(
        state: S,
        player: PlayerIndex<P>,
        next: impl NextGameTree<M, S, M, U, O, P>,
    ) -> Self {
        GameTree::new(state, Action::player(player, next))
    }

    /// Construct a game node where several players must make a move simultaneously and the next
    /// node is computed from the moves they choose.
    pub fn players(
        state: S,
        players: Vec<PlayerIndex<P>>,
        next: impl NextGameTree<Vec<M>, S, M, U, O, P>,
    ) -> Self {
        GameTree::new(state, Action::players(players, next))
    }

    /// Construct a game node where all players must make a move simultaneously and the next node
    /// is computed from the moves they choose.
    pub fn all_players(state: S, next: impl NextGameTree<Profile<M, P>, S, M, U, O, P>) -> Self {
        GameTree::new(state, Action::all_players(next))
    }

    /// Construct a game node where a move is selected from a distribution and the next node is
    /// computed from the selected move.
    pub fn chance(
        state: S,
        distribution: Distribution<M>,
        next: impl NextGameTree<M, S, M, U, O, P>,
    ) -> Self {
        GameTree::new(state, Action::chance(distribution, next))
    }

    /// Construct a game node ending the game with the given outcome.
    pub fn end(state: S, outcome: O) -> Self {
        GameTree::new(state, Action::end(outcome))
    }
}

impl<S, M: Move, U, O, const P: usize> Action<S, M, U, O, P> {
    /// Construct an action where a single player must make a move and the next node is computed
    /// from the move they choose.
    pub fn player(to_move: PlayerIndex<P>, next: impl NextGameTree<M, S, M, U, O, P>) -> Self {
        Action::players(vec![to_move], move |state, moves| {
            assert_eq!(moves.len(), 1);
            next(state, moves[0])
        })
    }

    /// Construct an action where several players must make a move simultaneously and the next node
    /// is computed from the moves they choose.
    pub fn players(
        to_move: Vec<PlayerIndex<P>>,
        next: impl NextGameTree<Vec<M>, S, M, U, O, P>,
    ) -> Self {
        Action::Turns {
            to_move,
            next: Arc::new(next),
        }
    }

    /// Construct an action where all players must make a move simultaneously and the next node is
    /// computed from the moves they choose.
    pub fn all_players(next: impl NextGameTree<Profile<M, P>, S, M, U, O, P>) -> Self {
        Action::players(PlayerIndex::all().collect(), move |state, moves| {
            assert_eq!(moves.len(), P);
            next(state, Profile::new(moves.try_into().unwrap()))
        })
    }

    /// Construct an action where a move is selected from a distribution and the next node is
    /// computed from the selected move.
    pub fn chance(
        distribution: Distribution<M>,
        next: impl NextGameTree<M, S, M, U, O, P>,
    ) -> Self {
        Action::Chance {
            distribution,
            next: Arc::new(next),
        }
    }

    /// Construct an action ending the game with the given outcome.
    pub fn end(outcome: O) -> Self {
        Action::End {
            outcome,
            utility_type: PhantomData,
        }
    }
}

impl<S: State, M: Move, U: Utility, O: Outcome<M, U, P>, const P: usize> Game<P>
    for GameTree<S, M, U, O, P>
{
    type Move = M;
    type Utility = U;
    type Outcome = O;
    type State = S;
    type View = S;

    fn state_view(&self, state: &Self::State, _player: PlayerIndex<P>) -> Self::View {
        state.clone()
    }
}

impl<S: State, M: Move, U: Utility, O: Outcome<M, U, P>, const P: usize> Playable<P>
    for GameTree<S, M, U, O, P>
{
    fn into_game_tree(self) -> GameTree<S, M, U, O, P> {
        self
    }
}
