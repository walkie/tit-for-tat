use crate::core::PlayerIndex;
use crate::game::Game;

pub trait Seq<const N: usize>: Game<N> {
    /// The type of state maintained while executing an iteration of this game.
    type State: Clone;

    /// Get the initial execution state for this game.
    fn initial_state(&self) -> Self::State;

    /// Is this a valid move for the given player at the given execution state?
    fn is_valid_move_for_player_at_state(
        &self,
        player: PlayerIndex<N>,
        state: &Self::State,
        the_move: Self::Move,
    ) -> bool;
}

pub trait FinSeq<const N: usize>: Seq<N> {}
