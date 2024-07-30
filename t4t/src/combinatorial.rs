use crate::{Move, Payoff, PlayResult, PlayerIndex, PossibleMoves, State, Utility};
use std::sync::Arc;

// TODO
#[allow(missing_docs)]

pub enum Combinatorial<S: State, M: Move, U: Utility, const P: usize> {
    Turn {
        state: S,
        turn: PlayerIndex<P>,
        moves: PossibleMoves<'static, M>,
        next: Arc<dyn Fn(M) -> PlayResult<Combinatorial<S, M, U, P>, S, M, P>>,
    },
    End {
        state: S,
        payoff: Payoff<U, P>,
    },
}
