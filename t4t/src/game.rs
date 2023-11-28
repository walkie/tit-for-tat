use std::fmt::Debug;

use crate::{Action, Context, Error, Move, Outcome, PlayerIndex, Players, Turn, Utility};

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

    fn play(&self, players: &mut Players<Self, P>) -> Result<Self::Outcome, Error<Self, P>> {
        let mut turn = self.rules();

        loop {
            match turn.action {
                Action::Players { to_move, next } => {
                    let moves = to_move
                        .iter()
                        .map(|&index| {
                            let view = self.state_view(turn.state.as_ref(), index);
                            let context = Context::new(index, view);
                            players[index].next_move(&context)
                        })
                        .collect();

                    match next(turn.state.clone(), moves) {
                        Ok(next_turn) => turn = next_turn,
                        Err(kind) => {
                            return Err(Error::new(turn.state, kind));
                        }
                    }
                }

                Action::Chance { distribution, next } => {
                    let the_move = distribution.sample();

                    match next(turn.state.clone(), *the_move) {
                        Ok(next_turn) => turn = next_turn,
                        Err(kind) => {
                            return Err(Error::new(turn.state, kind));
                        }
                    }
                }

                Action::End { outcome } => return Ok(outcome),
            }
        }
    }
}
