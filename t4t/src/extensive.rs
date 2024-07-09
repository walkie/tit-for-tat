use crate::{Distribution, Payoff, PlayerIndex};
use std::sync::Arc;

type Edge<M, U, const P: usize> = (M, Arc<Extensive<M, U, P>>);

// A game represented in [extensive form](https://en.wikipedia.org/wiki/Extensive-form_game).
pub enum Extensive<M, U, const P: usize> {
    Turn {
        to_move: PlayerIndex<P>,
        next: Vec<Edge<M, U, P>>,
    },
    Chance(Distribution<Edge<M, U, P>>),
    End(Payoff<U, P>),
}

/// An [information set](https://en.wikipedia.org/wiki/Information_set_(game_theory)) in an
/// imperfect information [extensive-form game](https://en.wikipedia.org/wiki/Extensive-form_game).
pub struct InformationSet<M, const P: usize> {
    id: usize,
    player: PlayerIndex<P>,
    available_moves: Vec<M>,
}
