use crate::per_player::PerPlayer;
use crate::play::{PlayState, Playable};
use crate::strategy::Strategy;

/// A [per-player](crate::PerPlayer) collection of [players](Player), ready to play a game.
pub type Players<Game, const N: usize> = PerPlayer<Player<Game, N>, N>;

/// A player consists of a name and a [strategy](crate::Strategy).
///
/// A player's name should usually be unique with respect to all players playing the same game.
pub struct Player<Game: Playable<N>, const N: usize> {
    name: String,
    strategy: Box<dyn Strategy<Game::Move, PlayState<Game, N>>>,
}

impl<Game: Playable<N>, const N: usize> Player<Game, N> {
    /// Construct a new player with the given name and strategy.
    pub fn new(
        name: String,
        strategy: impl Strategy<Game::Move, PlayState<Game, N>> + 'static,
    ) -> Self {
        Player {
            name,
            strategy: Box::new(strategy),
        }
    }

    /// The player's name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the player's next move to play given a particular game state.
    pub fn next_move(&mut self, state: &PlayState<Game, N>) -> Game::Move {
        self.strategy.next_move(state)
    }
}
