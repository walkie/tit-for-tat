use crate::{
    Game, GameTree, Payoff, PlayResult, Playable, PlayerIndex, SequentialOutcome, Transcript,
};
use std::sync::Arc;

/// A trait for defining [combinatorial games](https://en.wikipedia.org/wiki/Combinatorial_game_theory).
///
/// A combinatorial game is a [perfect-information](https://en.wikipedia.org/wiki/Perfect_information)
/// game where players interact by sequentially making moves to modify a shared state.
///
/// To define a combinatorial game, you should implement the [`Game`] trait and this trait. The
/// [`Playable`] trait is implemented generically for all instances of this trait.
pub trait Combinatorial<const P: usize>: Game<P> {
    /// The initial state of the game.
    fn initial_state(&self) -> Self::State;

    /// Given the state of the game, whose turn is it?
    fn whose_turn(&self, state: &Self::State) -> PlayerIndex<P>;

    /// Given the current state and the current player's move, produce the next state.
    fn next_state(
        &self,
        state: Self::State,
        the_move: Self::Move,
    ) -> PlayResult<Self::State, Self::State, Self::Move, P>;

    /// If the game is over, return the payoff. If the game is not over, return `None`.
    fn payoff(&self, state: &Self::State) -> Option<Payoff<Self::Utility, P>>;

    /// Is this game over? Returns `true` when the [`payoff`](Self::payoff) method returns `Some`,
    /// `false` otherwise.
    fn is_game_end(&self, state: &Self::State) -> bool {
        self.payoff(state).is_some()
    }
}

/// Generate the game tree for a combinatorial game.
#[allow(clippy::type_complexity)]
fn generate_tree<G: Combinatorial<P> + 'static, const P: usize>(
    game: Arc<G>,
    state: G::State,
    transcript: Transcript<G::Move, P>,
) -> GameTree<G::State, G::Move, G::Utility, SequentialOutcome<G::Move, G::Utility, P>, P> {
    let player = game.whose_turn(&state);
    match game.payoff(&state) {
        Some(payoff) => GameTree::end(state, SequentialOutcome::new(transcript, payoff)),

        None => GameTree::player(state, player, move |state, the_move| {
            let next_state = game.next_state(state, the_move)?;

            let mut updated_transcript = transcript.clone();
            updated_transcript.add_player_move(player, the_move);

            Ok(generate_tree(
                Arc::clone(&game),
                next_state,
                updated_transcript,
            ))
        }),
    }
}

impl<G: Combinatorial<P> + 'static, const P: usize> Playable<P> for G {
    type Outcome = SequentialOutcome<Self::Move, Self::Utility, P>;

    fn into_game_tree(self) -> GameTree<Self::State, Self::Move, Self::Utility, Self::Outcome, P> {
        let initial_state = self.initial_state();
        generate_tree(Arc::new(self), initial_state, Transcript::new())
    }
}
