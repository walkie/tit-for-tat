use std::fmt::Debug;

use crate::{Game, Payoff, PlayerIndex, Profile};

/// A [ply](https://en.wikipedia.org/wiki/Ply_(game_theory)) is a single move played during a game.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Ply<M, const P: usize> {
    /// The player that played the move, or `None` if it was a move of chance.
    pub player: Option<PlayerIndex<P>>,
    /// The move that was played.
    pub the_move: M,
}

pub enum Event<M, U, const P: usize> {
    Ply(Ply<M, P>),
    Payoff(Payoff<U, P>),
}

impl<M, const P: usize> Ply<M, P> {
    /// Construct a new played move.
    pub fn new(player: Option<PlayerIndex<P>>, the_move: M) -> Self {
        Ply { player, the_move }
    }

    /// Construct a move played by the given player.
    pub fn player(player: PlayerIndex<P>, the_move: M) -> Self {
        Ply::new(Some(player), the_move)
    }

    /// Construct a move played by chance.
    pub fn chance(the_move: M) -> Self {
        Ply::new(None, the_move)
    }

    /// Was this move played by a player (and not chance)?
    pub fn is_player(&self) -> bool {
        self.player.is_some()
    }

    /// Was this move played by chance?
    pub fn is_chance(&self) -> bool {
        self.player.is_none()
    }
}

impl<M, U, const P: usize> Event<M, U, P> {
    /// Construct an event corresponding to a player's move.
    pub fn player(player: PlayerIndex<P>, the_move: M) -> Self {
        Event::Ply(Ply::player(player, the_move))
    }

    /// Construct an event corresponding to a move by chance.
    pub fn chance(the_move: M) -> Self {
        Event::Ply(Ply::chance(the_move))
    }

    /// Construct an even corresponding to a payoff awarded to the players.
    pub fn payoff(payoff: Payoff<U, P>) -> Self {
        Event::Payoff(payoff)
    }

    /// Was this event a move made by a player?
    pub fn is_player_move(&self) -> bool {
        match self {
            Event::Ply(ply) => ply.is_player(),
            _ => false,
        }
    }

    /// Was this event a move made by chance?
    pub fn is_chance_move(&self) -> bool {
        match self {
            Event::Ply(ply) => ply.is_chance(),
            _ => false,
        }
    }

    /// Was this event a payoff?
    pub fn is_payoff(&self) -> bool {
        match self {
            Event::Payoff(_) => true,
            _ => false,
        }
    }
}
