use crate::moves::IsMove;
use crate::per_player::PerPlayer;
use crate::strategy::Strategy;

/// A [per-player](crate::PerPlayer) collection of [players](Player), ready to play a game.
pub type Players<Move, State, const N: usize> = PerPlayer<Player<Move, State>, N>;

/// A player consists of a name and a [strategy](crate::Strategy).
///
/// A player's name should ideally be unique with respect to all players playing the same game.
///
/// # Type variables
///
/// - `Move` -- The type of moves this player plays.
/// - `State` -- The type of the intermediate game state this player understands. Values of this
///   type may be used by the player's strategy to determine the player's next move.
pub struct Player<Move: IsMove, State> {
    name: String,
    strategy: Box<dyn Strategy<Move, State>>,
}

impl<Move: IsMove, State> Player<Move, State> {
    /// Construct a player with the given name and strategy.
    pub fn new<S>(name: String, strategy: S) -> Self
    where
        S: Strategy<Move, State> + 'static,
    {
        Player {
            name,
            strategy: Box::new(strategy),
        }
    }

    /// Get the player's name.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Query the player's strategy to get the next move for the given game state.
    pub fn next_move(&mut self, state: &State) -> Move {
        self.strategy.next_move(state)
    }
}
