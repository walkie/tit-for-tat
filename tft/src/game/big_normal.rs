use crate::prelude::*;

pub struct BigNormal<Move, Util, const N: usize> {
    moves: PerPlayer<Vec<Move>, N>,
    payoff_fn: Box<dyn Fn(Profile<Move, N>) -> Option<Payoff<Util, N>>>,
}

impl<Move: IsMove, Util: IsUtility, const N: usize> BigNormal<Move, Util, N> {
    pub fn new(
        moves: PerPlayer<Vec<Move>, N>,
        payoff_fn: impl Fn(Profile<Move, N>) -> Option<Payoff<Util, N>> + 'static,
    ) -> Self {
        BigNormal {
            moves,
            payoff_fn: Box::new(payoff_fn),
        }
    }
}

impl<Move: IsMove, Util: IsUtility, const N: usize> Game<N> for BigNormal<Move, Util, N> {
    type Move = Move;
    type Utility = Util;
    type State = ();

    fn initial_state(&self) {}

    fn is_valid_move_for_player_at_state(
        &self,
        player: PlayerIndex<N>,
        _state: &(),
        the_move: Move,
    ) -> bool {
        self.moves[player].contains(&the_move)
    }
}

impl<Move: IsMove, Util: IsUtility, const N: usize> Finite<N> for BigNormal<Move, Util, N> {
    fn available_moves_for_player_at_state(
        &self,
        player: PlayerIndex<N>,
        _state: &(),
    ) -> MoveIter<'_, Move> {
        MoveIter::new(self.moves[player].clone().into_iter())
    }
}

impl<Move: IsMove, Util: IsUtility, const N: usize> Simultaneous<N> for BigNormal<Move, Util, N> {
    fn payoff(&self, profile: Profile<Move, N>) -> Option<Payoff<Util, N>> {
        (*self.payoff_fn)(profile)
    }
}

impl<Move: IsMove, Util: IsUtility, const N: usize> FiniteSimultaneous<N>
    for BigNormal<Move, Util, N>
{
}
