use crate::{
    Game, GameTree, Payoff, PlayResult, Playable, PlayerIndex, SequentialOutcome, Transcript,
};
use std::sync::Arc;

// TODO
#[allow(missing_docs)]

pub trait Combinatorial<const P: usize>: Game<P> {
    fn initial_state(&self) -> Self::State;

    fn whose_turn(&self, state: &Self::State) -> PlayerIndex<P>;

    fn next_state(
        &self,
        state: Self::State,
        the_move: Self::Move,
    ) -> PlayResult<Self::State, Self::State, Self::Move, P>;

    fn payoff(&self, state: &Self::State) -> Option<Payoff<Self::Utility, P>>;

    fn is_game_end(&self, state: &Self::State) -> bool {
        self.payoff(state).is_some()
    }
}

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
