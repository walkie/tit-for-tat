//! General game trees.

use std::rc::Rc;

use crate::distribution::Distribution;
use crate::moves::IsMove;
use crate::normal::Normal;
use crate::outcome::Outcome;
use crate::payoff::{IsUtil, Payoff};
use crate::per_player::{PerPlayer, PlayerIndex};
use crate::simultaneous::Simultaneous;

#[derive(Clone)]
pub struct GameTree<Move, Util, State, const N: usize> {
    state: State,
    node: Node<Move, Util, State, N>,
}

#[derive(Clone)]
pub enum Node<Move, Util, State, const N: usize> {
    Turn {
        player: PlayerIndex<N>,
        moves: Moves<Move>,
        edges: Edges<Move, Util, State, N>,
    },
    Chance {
        distribution: Distribution<Move>,
        edges: Edges<Move, Util, State, N>,
    },
    Payoff {
        payoff: Payoff<Util, N>,
    },
}

pub type Edges<Move, Util, State, const N: usize> =
    Rc<dyn Fn(Move) -> Option<GameTree<Move, Util, State, N>>>;

#[derive(Clone)]
pub enum Moves<Move> {
    Discrete(Vec<Move>),
    Continuous(Rc<dyn Fn(Move) -> bool>),
}

impl<Move: IsMove, Util: IsUtil, State, const N: usize> GameTree<Move, Util, State, N> {
    pub fn new(state: State, node: Node<Move, Util, State, N>) -> Self {
        GameTree { state, node }
    }

    pub fn turn(
        state: State,
        player: PlayerIndex<N>,
        moves: Moves<Move>,
        edge_fn: impl Fn(Move) -> Option<GameTree<Move, Util, State, N>> + 'static,
    ) -> Self {
        GameTree::new(state, Node::turn(player, moves, edge_fn))
    }

    pub fn discrete_turn(
        state: State,
        player: PlayerIndex<N>,
        moves: Vec<Move>,
        edge_fn: impl Fn(Move) -> Option<GameTree<Move, Util, State, N>> + 'static,
    ) -> Self {
        GameTree::new(state, Node::discrete_turn(player, moves, edge_fn))
    }

    pub fn continuous_turn(
        state: State,
        player: PlayerIndex<N>,
        move_fn: impl Fn(Move) -> bool + 'static,
        edge_fn: impl Fn(Move) -> Option<GameTree<Move, Util, State, N>> + 'static,
    ) -> Self {
        GameTree::new(state, Node::continuous_turn(player, move_fn, edge_fn))
    }

    pub fn chance(
        state: State,
        distribution: Distribution<Move>,
        edge_fn: impl Fn(Move) -> Option<GameTree<Move, Util, State, N>> + 'static,
    ) -> Self {
        GameTree::new(state, Node::chance(distribution, edge_fn))
    }

    pub fn payoff(state: State, payoff: Payoff<Util, N>) -> Self {
        GameTree::new(state, Node::payoff(payoff))
    }

    pub fn state(&self) -> &State {
        &self.state
    }

    pub fn node(&self) -> &Node<Move, Util, State, N> {
        &self.node
    }
}

impl<Move: IsMove, Util: IsUtil, State, const N: usize> Node<Move, Util, State, N> {
    pub fn turn(
        player: PlayerIndex<N>,
        moves: Moves<Move>,
        edge_fn: impl Fn(Move) -> Option<GameTree<Move, Util, State, N>> + 'static,
    ) -> Self {
        Node::Turn {
            player,
            moves,
            edges: Rc::new(edge_fn),
        }
    }

    pub fn discrete_turn(
        player: PlayerIndex<N>,
        moves: Vec<Move>,
        edge_fn: impl Fn(Move) -> Option<GameTree<Move, Util, State, N>> + 'static,
    ) -> Self {
        Node::turn(player, Moves::Discrete(moves), edge_fn)
    }

    pub fn continuous_turn(
        player: PlayerIndex<N>,
        move_fn: impl Fn(Move) -> bool + 'static,
        edge_fn: impl Fn(Move) -> Option<GameTree<Move, Util, State, N>> + 'static,
    ) -> Self {
        Node::turn(player, Moves::Continuous(Rc::new(move_fn)), edge_fn)
    }

    pub fn chance(
        distribution: Distribution<Move>,
        edge_fn: impl Fn(Move) -> Option<GameTree<Move, Util, State, N>> + 'static,
    ) -> Self {
        Node::Chance {
            distribution,
            edges: Rc::new(edge_fn),
        }
    }

    pub fn payoff(payoff: Payoff<Util, N>) -> Self {
        Node::Payoff { payoff }
    }
}

impl<Move: IsMove> Moves<Move> {
    pub fn is_valid_move(&self, the_move: Move) -> bool {
        match self {
            Moves::Discrete(moves) => moves.contains(&the_move),
            Moves::Continuous(valid) => valid(the_move),
        }
    }
}
