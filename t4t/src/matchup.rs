use crate::{Game, PerPlayer, Player, Strategy};
use std::sync::Arc;

/// A collection of players ready to play a game.
#[derive(Clone, Debug)]
pub struct Matchup<G: Game<P>, const P: usize> {
    players: PerPlayer<Arc<Player<G, P>>, P>,
}

impl<G: Game<P>, const P: usize> Matchup<G, P> {
    /// Construct a new matchup.
    pub fn new(players: PerPlayer<Arc<Player<G, P>>, P>) -> Self {
        Matchup { players }
    }

    /// Construct a new matchup from an array of players.
    pub fn from_players(players: [Player<G, P>; P]) -> Self {
        Matchup::new(PerPlayer::new(players.map(Arc::new)))
    }

    /// Get the players in this matchup.
    pub fn players(&self) -> &PerPlayer<Arc<Player<G, P>>, P> {
        &self.players
    }

    /// Get the names of all players in this matchup.
    pub fn names(&self) -> PerPlayer<String, P> {
        self.players.map(|player| player.name().to_owned())
    }

    /// Get fresh copies of each player's strategy for playing the game.
    pub fn strategies(&self) -> PerPlayer<Strategy<G::View, G::Move, P>, P> {
        self.players.map(|player| player.new_strategy())
    }
}
