use crate::{Context, Game, GameTree, Matchup, Outcome, PlayResult};

/// A shared interface for playing games.
///
/// This trait is implemented by all game types. It defines how to convert the game into a
/// [`GameTree`], which can then be played by executing the game tree with a given matchup of
/// players.
pub trait Playable<const P: usize>: Game<P> + 'static {
    /// The type of value produced by playing the game.
    /// - For simultaneous games: [`SimultaneousOutcome`](crate::SimultaneousOutcome)
    /// - For sequential games: [`SequentialOutcome`](crate::SequentialOutcome)
    /// - For repeated games: [`History`](crate::History)
    type Outcome: Outcome<Self::Move, Self::Utility, P>;

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
    fn play(
        &self,
        matchup: &Matchup<Self, P>,
    ) -> PlayResult<Self::Outcome, Self::State, Self::Move, P> {
        let mut node = self.game_tree();
        let mut strategies = matchup.strategies();

        loop {
            match &node {
                GameTree::Turns {
                    state,
                    to_move,
                    next,
                } => {
                    let moves = to_move
                        .iter()
                        .map(|&index| {
                            let view = self.state_view(state, index);
                            let context = Context::new(index, &view, &node);
                            strategies[index].next_move(context)
                        })
                        .collect();

                    let next_node = next(state.clone(), moves)?;
                    node = next_node;
                }

                GameTree::Chance {
                    state,
                    distribution,
                    next,
                } => {
                    let the_move = distribution.sample();
                    let next_node = next(state.clone(), *the_move)?;
                    node = next_node;
                }

                GameTree::End { outcome, .. } => return Ok(outcome.clone()),
            }
        }
    }
}
