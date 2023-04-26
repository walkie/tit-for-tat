use crate::game::Game;
use crate::per_player::PerPlayer;
use crate::play::PlayState;
use crate::strategy::Strategy;

/// A [per-player](crate::PerPlayer) collection of [players](Player), ready to play a game.
pub type Players<G, const P: usize> = PerPlayer<Player<G, P>, P>;

/// A player consists of a name and a [strategy](crate::Strategy).
///
/// A player's name should usually be unique with respect to all players playing the same game.
pub struct Player<G: Game<P>, const P: usize> {
    name: String,
    strategy: Box<dyn Strategy<G, P>>,
}

impl<G: Game<P>, const P: usize> Player<G, P> {
    /// Construct a new player with the given name and strategy.
    pub fn new(name: String, strategy: impl Strategy<G, P> + 'static) -> Self {
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
    pub fn next_move(&mut self, state: &PlayState<G, P>) -> G::Move {
        self.strategy.next_move(state)
    }
}
