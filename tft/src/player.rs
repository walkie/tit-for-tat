use std::cell::RefCell;
use std::rc::Rc;

use crate::moves::IsMove;
use crate::per_player::PerPlayer;
use crate::strategy::Strategy;

/// A [per-player](crate::PerPlayer) collection of [players](Player), ready to play a game.
pub type Players<Move, State, const N: usize> = PerPlayer<Player<Move, State>, N>;

/// A player consists of a name and a [strategy](crate::Strategy).
///
/// A player's name should usually be unique with respect to all players playing the same game.
///
/// # Type variables
///
/// - `Move` -- The type of moves this player plays.
/// - `State` -- The type of the intermediate game state this player understands. Values of this
///   type may be used by the player's strategy to determine the player's next move.
#[derive(Clone)]
pub struct Player<Move: IsMove, State> {
    name: String,
    strategy: Rc<RefCell<dyn Strategy<Move, State>>>,
}

impl<Move: IsMove, State> Player<Move, State> {
    /// Construct a new player with the given name and strategy.
    pub fn new(name: String, strategy: impl Strategy<Move, State> + 'static) -> Self {
        Player {
            name,
            strategy: Rc::new(RefCell::new(strategy)),
        }
    }

    /// The player's name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the player's next move to play given a particular game state.
    pub fn next_move(&self, game_state: &State) -> Move {
        self.strategy.borrow_mut().next_move(game_state)
    }
}
