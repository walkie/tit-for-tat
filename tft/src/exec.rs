//! Definitions related to game execution.

pub(crate) mod history;

use crate::core::{IsMove, IsUtil, PerPlayer};
use crate::exec::history::{History, Transcript};

pub trait Game<const N: usize> {
    type Move: IsMove;
    type Util: IsUtil;

    fn num_players(&self) -> usize {
        N
    }
}

/// Sequential game.
pub trait SeqGame<const N: usize>: Game<N> {
    type State;
    fn initial_state(&self) -> Self::State;
}

pub struct SimIterState;

pub struct SeqIterState<G: SeqGame<N>, const N: usize> {
    /// State of the game in-progress.
    pub game_state: G::State,
    /// Transcript of moves played so far.
    pub transcript: Transcript<G::Move, N>,
}

pub struct Player<Move> {
    dummy: Move
}

pub struct ExecState<G: Game<N>, const N: usize> {
    game: G,
    players: PerPlayer<Player<G::Move>, N>,
    history: History<G::Move>,
}
