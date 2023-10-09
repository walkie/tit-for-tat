use std::rc::Rc;

use crate::{Distribution, Move, Payoff, PlayerIndex, Utility};

#[derive(Clone)]
pub struct GameTree<M, U, S, const P: usize> {
    state: S,
    node: Node<M, U, S, P>,
}

#[derive(Clone)]
pub enum Node<M, U, S, const P: usize> {
    Turn {
        player: PlayerIndex<P>,
        moves: Moves<M>,
        edges: Edges<M, U, S, P>,
    },
    Chance {
        distribution: Distribution<M>,
        edges: Edges<M, U, S, P>,
    },
    Payoff {
        payoff: Payoff<U, P>,
    },
}

/// The moves available from a position in a game.
#[derive(Clone)]
pub enum Moves<M> {
    /// A finite domain of moves.
    Finite(Vec<M>),
    NonFinite(Rc<dyn Fn(M) -> bool>),
}

pub type Edges<M, U, S, const P: usize> = Rc<dyn Fn(M) -> Option<GameTree<M, U, S, P>>>;

impl<M: Move, U: Utility, S, const P: usize> GameTree<M, U, S, P> {
    pub fn new(state: S, node: Node<M, U, S, P>) -> Self {
        GameTree { state, node }
    }

    pub fn turn(
        state: S,
        player: PlayerIndex<P>,
        moves: Moves<M>,
        edge_fn: impl Fn(M) -> Option<GameTree<M, U, S, P>> + 'static,
    ) -> Self {
        GameTree::new(state, Node::turn(player, moves, edge_fn))
    }

    pub fn turn_finite(
        state: S,
        player: PlayerIndex<P>,
        moves: Vec<M>,
        edge_fn: impl Fn(M) -> Option<GameTree<M, U, S, P>> + 'static,
    ) -> Self {
        GameTree::new(state, Node::turn_finite(player, moves, edge_fn))
    }

    pub fn turn_nonfinite(
        state: S,
        player: PlayerIndex<P>,
        move_fn: impl Fn(M) -> bool + 'static,
        edge_fn: impl Fn(M) -> Option<GameTree<M, U, S, P>> + 'static,
    ) -> Self {
        GameTree::new(state, Node::turn_nonfinite(player, move_fn, edge_fn))
    }

    pub fn chance(
        state: S,
        distribution: Distribution<M>,
        edge_fn: impl Fn(M) -> Option<GameTree<M, U, S, P>> + 'static,
    ) -> Self {
        GameTree::new(state, Node::chance(distribution, edge_fn))
    }

    pub fn payoff(state: S, payoff: Payoff<U, P>) -> Self {
        GameTree::new(state, Node::payoff(payoff))
    }

    pub fn state(&self) -> &S {
        &self.state
    }

    pub fn node(&self) -> &Node<M, U, S, P> {
        &self.node
    }
}

impl<M: Move, U: Utility, S, const P: usize> Node<M, U, S, P> {
    pub fn turn(
        player: PlayerIndex<P>,
        moves: Moves<M>,
        edge_fn: impl Fn(M) -> Option<GameTree<M, U, S, P>> + 'static,
    ) -> Self {
        Node::Turn {
            player,
            moves,
            edges: Rc::new(edge_fn),
        }
    }

    pub fn turn_finite(
        player: PlayerIndex<P>,
        moves: Vec<M>,
        edge_fn: impl Fn(M) -> Option<GameTree<M, U, S, P>> + 'static,
    ) -> Self {
        Node::turn(player, Moves::Finite(moves), edge_fn)
    }

    pub fn turn_nonfinite(
        player: PlayerIndex<P>,
        move_fn: impl Fn(M) -> bool + 'static,
        edge_fn: impl Fn(M) -> Option<GameTree<M, U, S, P>> + 'static,
    ) -> Self {
        Node::turn(player, Moves::NonFinite(Rc::new(move_fn)), edge_fn)
    }

    pub fn chance(
        distribution: Distribution<M>,
        edge_fn: impl Fn(M) -> Option<GameTree<M, U, S, P>> + 'static,
    ) -> Self {
        Node::Chance {
            distribution,
            edges: Rc::new(edge_fn),
        }
    }

    pub fn payoff(payoff: Payoff<U, P>) -> Self {
        Node::Payoff { payoff }
    }
}

impl<M: Move> Moves<M> {
    pub fn is_valid_move(&self, the_move: M) -> bool {
        match self {
            Moves::Finite(moves) => moves.contains(&the_move),
            Moves::NonFinite(valid) => valid(the_move),
        }
    }
}
