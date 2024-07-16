use crate::{
    ErrorKind, Game, GameTree, Payoff, Playable, PlayerIndex, SequentialOutcome, Transcript,
};
use std::sync::Arc;

pub trait StateBased<const P: usize>: Game<P> {
    fn initial_state(&self) -> Self::State;

    fn next_turn(&self, state: &Self::State) -> PlayerIndex<P>;

    fn next_state(
        &self,
        state: Self::State,
        player: PlayerIndex<P>,
        the_move: Self::Move,
    ) -> Result<Self::State, ErrorKind<Self::Move, P>>;

    fn check_final_state(
        &self,
        state: &Self::State,
        player: PlayerIndex<P>,
    ) -> Option<Payoff<Self::Utility, P>>;

    fn is_final_state(&self, state: &Self::State, player: PlayerIndex<P>) -> bool {
        self.check_final_state(state, player).is_some()
    }
}

fn generate_tree<G: StateBased<P> + 'static, const P: usize>(
    game: Arc<G>,
    state: G::State,
    transcript: Transcript<G::Move, P>,
) -> GameTree<G::State, G::Move, G::Utility, SequentialOutcome<G::Move, G::Utility, P>, P> {
    let player = game.next_turn(&state);
    match game.check_final_state(&state, player) {
        Some(payoff) => GameTree::end(state, SequentialOutcome::new(transcript, payoff)),

        None => GameTree::player(state, player, move |state, the_move| {
            let maybe_next_state = game.next_state(state, player, the_move);

            let mut updated_transcript = transcript.clone();
            updated_transcript.add_player_move(player, the_move);

            match maybe_next_state {
                Ok(next_state) => Ok(generate_tree(
                    Arc::clone(&game),
                    next_state,
                    updated_transcript,
                )),

                Err(kind) => Err(kind),
            }
        }),
    }
}

impl<G: StateBased<P> + 'static, const P: usize> Playable<P> for G {
    type Outcome = SequentialOutcome<Self::Move, Self::Utility, P>;

    fn into_game_tree(self) -> GameTree<Self::State, Self::Move, Self::Utility, Self::Outcome, P> {
        let initial_state = self.initial_state();
        generate_tree(Arc::new(self), initial_state, Transcript::new())
    }
}
