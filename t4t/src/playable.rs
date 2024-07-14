use crate::{Action, Context, Error, Game, GameTree, Matchup};

/// The result of playing a game. Either an outcome or an error.
pub type PlayResult<G, const P: usize> =
    Result<<G as Game<P>>::Outcome, Error<<G as Game<P>>::State, <G as Game<P>>::Move, P>>;

/// A shared interface for playing games.
///
/// This trait is implemented by all game types. It defines how to convert the game into a
/// [`GameTree`], which can then be played by executing the game tree with a given matchup of
/// players.
pub trait Playable<const P: usize>: Game<P> {
    /// Convert this game into the corresponding game tree.
    ///
    /// The game tree is effectively a step-by-step executable description of how this game is
    /// played.
    fn into_game_tree(self) -> GameTree<Self::State, Self::Move, Self::Utility, Self::Outcome, P>;

    /// Get the corresponding game tree for this game.
    ///
    /// The game tree is effectively a step-by-step executable description of how this game is
    /// played.
    fn game_tree(&self) -> GameTree<Self::State, Self::Move, Self::Utility, Self::Outcome, P> {
        self.clone().into_game_tree()
    }

    /// Play this game with the given players by executing the game tree.
    ///
    /// Produces a value of the game's outcome type on success, otherwise an error.
    fn play(&self, matchup: &Matchup<Self, P>) -> PlayResult<Self, P> {
        let mut node = self.game_tree();
        let mut strategies = matchup.strategies();

        loop {
            match node.action {
                Action::Turns { to_move, next } => {
                    let moves = to_move
                        .iter()
                        .map(|&index| {
                            let view = self.state_view(&node.state, index);
                            let context = Context::new(index, view);
                            strategies[index].next_move(&context)
                        })
                        .collect();

                    match next(node.state.clone(), moves) {
                        Ok(next_node) => node = next_node,
                        Err(kind) => {
                            return Err(Error::new(node.state, kind));
                        }
                    }
                }

                Action::Chance { distribution, next } => {
                    let the_move = distribution.sample();

                    match next(node.state.clone(), *the_move) {
                        Ok(next_node) => node = next_node,
                        Err(kind) => {
                            return Err(Error::new(node.state, kind));
                        }
                    }
                }

                Action::End { outcome, .. } => return Ok(outcome),
            }
        }
    }
}
