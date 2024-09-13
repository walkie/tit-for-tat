use crate::{
    Distribution, Game, Move, Outcome, PlayResult, Playable, PlayerIndex, Profile, State, Utility,
};
use itertools::Itertools;
use std::marker::PhantomData;
use std::sync::Arc;

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

impl<S: State, M: Move, U: Utility, O: Outcome<M, U, P>, const P: usize> GameTree<S, M, U, O, P> {
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

    /// Transform the game tree such that each [GameTree::Turns] node contains exactly one player.
    /// Simultaneous turns will become sequential using an arbitrary ordering of players.
    pub fn sequentialize(self) -> Self {
        match self {
            GameTree::Turns {
                state,
                to_move,
                next,
            } => {
                if to_move.is_empty() {
                    next(state, vec![])
                        .map(|node| node.sequentialize())
                        .expect("Turns node with no players failed to produce next node")
                } else {
                    Self::sequentialize_turns(
                        state,
                        to_move.into_iter().rev().collect_vec(),
                        vec![],
                        next,
                    )
                }
            }
            GameTree::Chance {
                state,
                distribution,
                next,
            } => {
                let new_next =
                    move |state, the_move| next(state, the_move).map(|node| node.sequentialize());
                GameTree::chance(state, distribution, new_next)
            }
            GameTree::End { state, outcome, .. } => GameTree::end(state, outcome),
        }
    }

    fn sequentialize_turns(
        state: S,
        still_to_move: Vec<PlayerIndex<P>>,
        moves_so_far: Vec<M>,
        original_next: Arc<dyn NextGameTree<Vec<M>, S, M, U, O, P>>,
    ) -> GameTree<S, M, U, O, P> {
        assert!(!still_to_move.is_empty());
        if still_to_move.len() == 1 {
            GameTree::player(state, still_to_move[0], move |state, the_move| {
                let mut moves = moves_so_far.clone();
                moves.push(the_move);
                original_next(state, moves).map(|node| node.sequentialize())
            })
        } else {
            GameTree::player(
                state,
                *still_to_move.last().unwrap(),
                move |state, the_move| {
                    let mut moves_so_far = moves_so_far.clone();
                    let mut still_to_move = still_to_move.clone();
                    moves_so_far.push(the_move);
                    still_to_move.pop();
                    Ok(Self::sequentialize_turns(
                        state,
                        still_to_move,
                        moves_so_far,
                        Arc::clone(&original_next),
                    ))
                },
            )
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Matchup, Normal, Payoff, PerPlayer, Player, SimultaneousOutcome, Strategy};
    use impls::impls;
    use test_log::test;

    #[test]
    fn game_tree_is_send_sync() {
        assert!(impls!(GameTree<(), (), u8, SimultaneousOutcome<(), u8, 2>, 2>: Send & Sync));
    }

    #[test]
    fn game_tree_sequentialize() {
        let moves1 = vec!['A', 'B', 'C'];
        let moves2 = vec!['D', 'E'];
        let moves3 = vec!['F', 'G'];

        let simultaneous = Normal::from_payoff_vec(
            PerPlayer::new([moves1.clone(), moves2.clone(), moves3.clone()]),
            vec![
                Payoff::from([0, 1, 2]),
                Payoff::from([1, 2, 3]),
                Payoff::from([2, 3, 4]),
                Payoff::from([3, 4, 5]),
                Payoff::from([4, 5, 6]),
                Payoff::from([5, 6, 7]),
                Payoff::from([6, 7, 8]),
                Payoff::from([7, 8, 9]),
                Payoff::from([8, 9, 10]),
                Payoff::from([9, 10, 11]),
                Payoff::from([10, 11, 12]),
                Payoff::from([11, 12, 13]),
            ],
        )
        .unwrap()
        .game_tree();

        let sequential = simultaneous.clone().sequentialize();

        for m1 in moves1 {
            for m2 in moves2.clone() {
                for m3 in moves3.clone() {
                    let p1 = Player::new("P1".to_string(), move || Strategy::pure(m1));
                    let p2 = Player::new("P2".to_string(), move || Strategy::pure(m2));
                    let p3 = Player::new("P3".to_string(), move || Strategy::pure(m3));

                    let simultaneous_outcome = simultaneous
                        .play(&Matchup::from_players([p1.clone(), p2.clone(), p3.clone()]))
                        .unwrap();

                    let sequential_outcome = sequential
                        .play(&Matchup::from_players([p1, p2, p3]))
                        .unwrap();

                    assert_eq!(simultaneous_outcome, sequential_outcome);
                }
            }
        }
    }
}
