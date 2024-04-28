use std::fmt::Debug;

use crate::{Action, Context, Error, Move, Outcome, PlayerIndex, Players, Turn, Utility};

/// A trait that collects the trait requirements of a game state.
///
/// A blanket implementation covers all types that meet the requirements, so this trait should not
/// be implemented directly.
pub trait State: Clone + Debug + PartialEq + 'static {}
impl<T: Clone + Debug + PartialEq + 'static> State for T {}

/// A root trait that all games implement, mostly used for its associated types.
pub trait Game<const P: usize>: Sized {
    /// The type of moves played by players in this game.
    type Move: Move;

    /// The type of utility values awarded to each player at the end of the game.
    type Utility: Utility;

    /// The type of value produced by playing the game.
    /// - For simultaneous games: [SimultaneousOutcome](crate::SimultaneousOutcome)
    /// - For sequential games: [SequentialOutcome](crate::SequentialOutcome)
    /// - For repeated games: [History](crate::History)
    type Outcome: Outcome<Self::Move, Self::Utility, P>;

    /// The type of intermediate game state used during the execution of the game.
    type State: State;

    /// The type of the *view* of the intermediate game state presented to players.
    ///
    /// This may differ from [State] to support hidden information, that is, aspects of the game
    /// state that are not visible to players while making strategic decisions.
    type View: State;

    /// The first turn in the specification of the execution of this game.
    fn first_turn(&self) -> Turn<Self::State, Self::Move, Self::Outcome, P>;

    /// Produce a view of the game state for the given player.
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

    /// Play this game with the given players, producing a value of the game's outcome type on
    /// success.
    fn play(
        &self,
        players: &mut Players<Self, P>,
    ) -> Result<Self::Outcome, Error<Self::State, Self::Move, P>> {
        let mut turn = self.first_turn();

        loop {
            match turn.action {
                Action::Players { to_move, next } => {
                    let moves = to_move
                        .iter()
                        .map(|&index| {
                            let view = self.state_view(&turn.state, index);
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
