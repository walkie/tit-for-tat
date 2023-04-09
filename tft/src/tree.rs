//! General game trees.

use std::rc::Rc;

use crate::distribution::Distribution;
use crate::moves::IsMove;
use crate::normal::Normal;
use crate::outcome::Outcome;
use crate::payoff::{IsUtil, Payoff};
use crate::per_player::{PerPlayer, PlayerIndex};
use crate::player::Players;
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
        moves: Option<Vec<Move>>,
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

impl<Move: IsMove, Util: IsUtil, State, const N: usize> Node<Move, Util, State, N> {
    pub fn turn(
        player: PlayerIndex<N>,
        moves: Option<Vec<Move>>,
        edges: Edges<Move, Util, State, N>,
    ) -> Self {
        Node::Turn {
            player,
            moves,
            edges,
        }
    }

    pub fn discrete_turn(
        player: PlayerIndex<N>,
        moves: Vec<Move>,
        edges: Edges<Move, Util, State, N>,
    ) -> Self {
        Node::turn(player, Some(moves), edges)
    }

    pub fn continuous_turn(player: PlayerIndex<N>, edges: Edges<Move, Util, State, N>) -> Self {
        Node::turn(player, None, edges)
    }

    pub fn chance(distribution: Distribution<Move>, edges: Edges<Move, Util, State, N>) -> Self {
        Node::Chance {
            distribution,
            edges,
        }
    }

    pub fn payoff(payoff: Payoff<Util, N>) -> Self {
        Node::Payoff { payoff }
    }
}

pub type Edges<Move, Util, State, const N: usize> =
    Rc<dyn Fn(Move) -> Option<GameTree<Move, Util, State, N>>>;

impl<Move: IsMove, Util: IsUtil, State, const N: usize> GameTree<Move, Util, State, N> {
    pub fn turn(
        state: State,
        player: PlayerIndex<N>,
        moves: Option<Vec<Move>>,
        edges: Edges<Move, Util, State, N>,
    ) -> Self {
        GameTree {
            state,
            node: Node::turn(player, moves, edges),
        }
    }

    pub fn discrete_turn(
        state: State,
        player: PlayerIndex<N>,
        moves: Vec<Move>,
        edges: Edges<Move, Util, State, N>,
    ) -> Self {
        GameTree {
            state,
            node: Node::turn(player, Some(moves), edges),
        }
    }

    pub fn continuous_turn(
        state: State,
        player: PlayerIndex<N>,
        edges: Edges<Move, Util, State, N>,
    ) -> Self {
        GameTree {
            state,
            node: Node::turn(player, None, edges),
        }
    }

    pub fn chance(
        state: State,
        distribution: Distribution<Move>,
        edges: Edges<Move, Util, State, N>,
    ) -> Self {
        GameTree {
            state,
            node: Node::chance(distribution, edges),
        }
    }

    pub fn payoff(state: State, payoff: Payoff<Util, N>) -> Self {
        GameTree {
            state,
            node: Node::payoff(payoff),
        }
    }
}
