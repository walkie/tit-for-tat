use thiserror::Error;

use crate::context::Context;
use crate::game::Game;
use crate::history::{GameRecord, History};
use crate::payoff::Payoff;
use crate::per_player::PlayerIndex;
use crate::player::Players;
use crate::transcript::Transcript;

/// An error during game execution.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq, Hash)]
pub enum Error<G: Game<P>, const P: usize> {
    /// A player played an invalid move.
    InvalidMove(PlayerIndex<P>, G::Move),

    /// An apparently valid move did not produce the next node in the game tree. This is likely an
    /// error in the construction of the game.
    MalformedGame(G::Move),
}

pub struct Engine<G: Game<P>, const P: usize> {
    game: G,
    context: Context<G, P>,
    players: Players<G, P>,
}
