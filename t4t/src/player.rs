use crate::{Playable, Strategy};
use std::sync::Arc;

/// A player consists of a name and a function that produces its [strategy](Strategy).
///
/// A player's name must be unique with respect to all other players playing the same game (e.g.
/// in a tournament).
#[derive(Clone)]
pub struct Player<G: Playable<P>, const P: usize> {
    name: String,
    new_strategy: Arc<dyn Fn() -> Strategy<G, P> + Send + Sync>,
}

impl<G: Playable<P>, const P: usize> Player<G, P> {
    /// Construct a new player with the given name and a function to produce their strategy.
    pub fn new(
        name: String,
        new_strategy: impl Fn() -> Strategy<G, P> + Send + Sync + 'static,
    ) -> Self {
        Player {
            name,
            new_strategy: Arc::new(new_strategy),
        }
    }

    /// The player's name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get a new copy of this player's strategy for playing a game.
    pub fn new_strategy(&self) -> Strategy<G, P> {
        (self.new_strategy)()
    }
}

impl<G: Playable<P>, const P: usize> std::fmt::Debug for Player<G, P> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "Player({})", self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Normal;
    use impls::impls;
    use test_log::test;

    #[test]
    fn player_is_send_sync() {
        assert!(impls!(Player<Normal<(), u8, 2>, 2>: Send & Sync));
    }
}
