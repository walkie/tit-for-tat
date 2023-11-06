use std::fmt::Debug;

use crate::{
    Action, Context, Exec, Move, Outcome, Payoff, PlayerIndex, Players, Profile, Turn, Utility,
};

pub trait State: Clone + Debug + PartialEq {}
impl<T: Clone + Debug + PartialEq> State for T {}

/// A root trait that all games implement, mostly used for its associated types.
pub trait Game<const P: usize>: Sized {
    /// The type of moves played by players in this game.
    type Move: Move;

    /// The type of utility values awarded to each player at the end of the game.
    type Utility: Utility;

    type Outcome: Outcome<Self::Move, Self::Utility, P>;

    type State: State;

    type View: State;

    /// The first turn of the game.
    fn rules(&self) -> Turn<Self, P>;

    fn state_view(&self, state: &Self::State, player: PlayerIndex<P>) -> Self::View;

    /// Is this a valid move in the given context?
    fn is_valid_move(
        &self,
        state: &Self::State,
        player: PlayerIndex<P>,
        the_move: Self::Move,
    ) -> bool;

    /// The number of players this game is for.
    fn num_players(&self) -> usize {
        P
    }

    fn play(&self, players: Players<Self, P>) -> Self::Outcome {
        let mut turn = self.rules();

        loop {
            match &turn.action {
                Action::Players { players, next } => {
                    let moves = players
                        .map(|player| {
                            let context = Context::default(); // TODO
                            player.next_move(&context)
                        })
                        .collect();
                    turn = next(turn.state, moves);
                }

                Action::Chance { distribution, next } => {
                    let the_move = distribution.sample();
                    turn = next(turn.state, the_move);
                }

                Action::Payoff { payoff, outcome } => outcome(turn.state, *payoff),
            }
        }
    }
}
