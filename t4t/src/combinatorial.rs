use std::sync::Arc;
use crate::{ErrorKind, Game, Move, Payoff, PlayerIndex, State};

pub struct Combinatorial<S: State, M: Move, const P: usize> {
    state: S,
    turn: PlayerIndex<P>,
    moves: Vec<M>,
    next: Arc<dyn Fn(Combinatorial<S, M, P>, Vec<M>) -> Result<Combinatorial<S, M, P>, ErrorKind<M, P>>>,
}

pub struct CombinatorialState<S: State, M: Move, const P: usize> {
    state: S,
    turn: PlayerIndex<P>,
    moves: Vec<M>,
    next: Arc<dyn Fn(Combinatorial<S, M, P>, Vec<M>) -> Result<Combinatorial<S, M, P>, ErrorKind<M, P>>>,
    
    fn next_turn(&self, state: &Self::State) -> PlayerIndex<P>;

    fn next_state(
        &self,
        player: PlayerIndex<P>,
        the_move: Self::Move,
        state: Self::State,
    ) -> Result<Self::State, ErrorKind<Self::Move, P>>;

    fn check_final_state(
        &self,
        player: PlayerIndex<P>,
        state: &Self::State,
    ) -> Option<Payoff<Self::Utility, P>>;

    fn is_final_state(&self, player: PlayerIndex<P>, state: &Self::State) -> bool {
        self.check_final_state(player, state).is_some()
    }
}
