use std::marker::PhantomData;
use std::sync::Arc;

use crate::{
    Distribution, Game, Move, Outcome, PlayResult, Playable, PlayerIndex, Profile, State, Utility,
};

/// The outgoing edges of a node in a game tree, represented as a function.
///
/// The function yields the next node in the tree, given the current game state and the result of
/// this node's action.
///
/// This trait is effectively a type synonym for the function type it extends. A blanket
/// implementation covers all possible instances, so it should not be implemented directly.
pub trait NextGameTree<T, S, M, U, O, const P: usize>:
    Fn(S, T) -> PlayResult<GameTree<S, M, U, O, P>, S, M, P> + Send + Sync + 'static
{
}

impl<F, T, S, M, U, O, const P: usize> NextGameTree<T, S, M, U, O, P> for F where
    F: Fn(S, T) -> PlayResult<GameTree<S, M, U, O, P>, S, M, P> + Send + Sync + 'static
{
}

/// A node in a game tree. The current game state and an [action](GameTreeAction) to perform.
///
/// Subsequent nodes, if applicable, are reachable via the action's `next` function.
// #[derive(Clone)]
// pub struct GameTree<S, M, U, O, const P: usize> {
//     /// The game state at this node.
//     pub state: S,
//     /// The action to take at this node.
//     pub action: GameTreeAction<S, M, U, O, P>,
// }

/// A node in a game tree.
#[derive(Clone)]
pub enum GameTree<S, M, U, O, const P: usize> {
    /// One or more players play a move simultaneously.
    Turns {
        /// The game state at this node.
        state: S,
        /// The players to move simultaneously.
        to_move: Vec<PlayerIndex<P>>,
        /// Compute the next node in the tree from the moves played by the players.
        next: Arc<dyn NextGameTree<Vec<M>, S, M, U, O, P>>,
    },

    /// Make a move of chance according to the given distribution.
    Chance {
        /// The game state at this node.
        state: S,
        /// The distribution to draw a move from.
        distribution: Distribution<M>,
        /// Compute the next node in the tree from the move drawn from the distribution.
        next: Arc<dyn NextGameTree<M, S, M, U, O, P>>,
    },

    /// End a game and return the outcome, which includes the game's payoff.
    End {
        /// The final game state.
        state: S,
        /// The final outcome of the game.
        outcome: O,
        /// Phantom data to specify the utility value type.
        utility_type: PhantomData<U>,
    },
}

impl<S, M: Move, U, O, const P: usize> GameTree<S, M, U, O, P> {
    /// Construct a game node where a single player must make a move and the next node is computed
    /// from the move they choose.
    pub fn player(
        state: S,
        to_move: PlayerIndex<P>,
        next: impl NextGameTree<M, S, M, U, O, P>,
    ) -> Self {
        GameTree::players(state, vec![to_move], move |state, moves| {
            assert_eq!(moves.len(), 1);
            next(state, moves[0])
        })
    }

    /// Construct a game node where several players must make a move simultaneously and the next
    /// node is computed from the moves they choose.
    pub fn players(
        state: S,
        to_move: Vec<PlayerIndex<P>>,
        next: impl NextGameTree<Vec<M>, S, M, U, O, P>,
    ) -> Self {
        GameTree::Turns {
            state,
            to_move,
            next: Arc::new(next),
        }
    }

    /// Construct a game node where all players must make a move simultaneously and the next node
    /// is computed from the moves they choose.
    pub fn all_players(state: S, next: impl NextGameTree<Profile<M, P>, S, M, U, O, P>) -> Self {
        GameTree::players(state, PlayerIndex::all().collect(), move |state, moves| {
            assert_eq!(moves.len(), P);
            next(state, Profile::new(moves.try_into().unwrap()))
        })
    }

    /// Construct a game node where a move is selected from a distribution and the next node is
    /// computed from the selected move.
    pub fn chance(
        state: S,
        distribution: Distribution<M>,
        next: impl NextGameTree<M, S, M, U, O, P>,
    ) -> Self {
        GameTree::Chance {
            state,
            distribution,
            next: Arc::new(next),
        }
    }

    /// Construct a game node ending the game with the given outcome.
    pub fn end(state: S, outcome: O) -> Self {
        GameTree::End {
            state,
            outcome,
            utility_type: PhantomData,
        }
    }

    /// Get the game state at this node.
    pub fn state(&self) -> &S {
        match self {
            GameTree::Turns { state, .. } => state,
            GameTree::Chance { state, .. } => state,
            GameTree::End { state, .. } => state,
        }
    }
}

impl<S: State, M: Move, U: Utility, O: Outcome<M, U, P>, const P: usize> Game<P>
    for GameTree<S, M, U, O, P>
{
    type Move = M;
    type Utility = U;
    type State = S;
    type View = S;

    fn state_view(&self, state: &Self::State, _player: PlayerIndex<P>) -> Self::View {
        state.clone()
    }
}

impl<S: State, M: Move, U: Utility, O: Outcome<M, U, P>, const P: usize> Playable<P>
    for GameTree<S, M, U, O, P>
{
    type Outcome = O;

    fn into_game_tree(self) -> GameTree<S, M, U, O, P> {
        self
    }
}
