use crate::moves::Move;
use crate::per_player::PerPlayer;
use crate::strategy::Strategy;

/// A [per-player](crate::PerPlayer) collection of [players](Player), ready to play a game.
pub type Players<C, M, const P: usize> = PerPlayer<Player<C, M>, P>;

/// A player consists of a name and a [strategy](crate::Strategy).
///
/// A player's name should usually be unique with respect to all players playing the same game.
pub struct Player<C, M: Move> {
    name: String,
    strategy: Box<dyn Strategy<C, M>>,
}

impl<C, M: Move> Player<C, M> {
    /// Construct a new player with the given name and strategy.
    pub fn new(name: String, strategy: impl Strategy<C, M> + 'static) -> Self {
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
    pub fn next_move(&mut self, context: &C) -> M {
        self.strategy.next_move(context)
    }
}
