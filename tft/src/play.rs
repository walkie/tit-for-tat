use crate::moves::IsMove;
use crate::normal::Normal;
use crate::outcome::Outcome;
use crate::payoff::{IsUtil, Payoff};
use crate::per_player::{PerPlayer, PlayerIndex};
use crate::simultaneous::Simultaneous;
use crate::strategy::Strategy;
use crate::tree::*;

/// A [per-player](crate::PerPlayer) collection of [players](Player), ready to play a game.
pub type Players<Move, State, const N: usize> = PerPlayer<Player<Move, State>, N>;

/// A player consists of a name and a [strategy](crate::Strategy).
///
/// A player's name should ideally be unique with respect to all players playing the same game.
///
/// # Type variables
///
/// - `Move` -- The type of moves this player plays.
/// - `State` -- The type of the intermediate game state this player understands. Values of this
///   type may be used by the player's strategy to determine the player's next move.
pub struct Player<Move: IsMove, State> {
    name: String,
    strategy: Box<dyn Strategy<Move, State>>,
}

impl<Move: IsMove, State> Player<Move, State> {
    /// Construct a player with the given name and strategy.
    pub fn new<S>(name: String, strategy: S) -> Self
    where
        S: Strategy<Move, State> + 'static,
    {
        Player {
            name,
            strategy: Box::new(strategy),
        }
    }

    /// Get the player's name.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Query the player's strategy to get the next move for the given game state.
    pub fn next_move(&mut self, state: &State) -> Move {
        self.strategy.next_move(state)
    }
}

/// An interface for playing games.
pub trait Playable<Move: IsMove, Util: IsUtil, State: Clone, const N: usize> {
    fn to_game_tree(&self) -> GameTree<Move, Util, State, N>;
    fn play(&self, players: &mut Players<Move, State, N>) -> PlayResult<Move, Util, State, N> {
        self.to_game_tree().play(players)
    }
}

/// The result of playing a game. Includes the final state and either the payoff resulting from a
/// successfully completed game, or an error.
pub enum PlayResult<Move, Util, State, const N: usize> {
    Done(State, Payoff<Util, N>),
    Error(State, PlayError<Move, N>),
}

/// An error during game execution.
pub enum PlayError<Move, const N: usize> {
    /// A player played an invalid move.
    InvalidMove(PlayerIndex<N>, Move),

    /// An apparently valid move did not produce the next node in the game tree. This is likely an
    /// error in the construction of the game.
    MalformedGame(Move),
}

impl<Move: IsMove, Util: IsUtil, State: Clone, const N: usize> GameTree<Move, Util, State, N> {
    pub fn play(&self, players: &mut Players<Move, State, N>) -> PlayResult<Move, Util, State, N> {
        let mut current = self.clone();
        loop {
            let (to_play, edges) = match current.node() {
                Node::Turn {
                    player,
                    moves,
                    edges,
                } => {
                    let to_play = players[*player].next_move(self.state());
                    if !moves.is_valid_move(to_play) {
                        return PlayResult::Error(
                            current.state().clone(),
                            PlayError::InvalidMove(*player, to_play),
                        );
                    }
                    (to_play, edges)
                }
                Node::Chance {
                    distribution,
                    edges,
                } => (*distribution.sample(), edges),
                Node::Payoff { payoff } => {
                    return PlayResult::Done(current.state().clone(), *payoff)
                }
            };
            if let Some(next) = edges(to_play) {
                current = next;
            } else {
                return PlayResult::Error(
                    current.state().clone(),
                    PlayError::MalformedGame(to_play),
                );
            }
        }
    }
}

// impl<Move: IsMove, Util: IsUtil, const N: usize> Playable<N> for Simultaneous<Move, Util, N> {
//     type Move = Move;
//     type Util = Util;
//     type Outcome = Outcome<Move, Util, N>;
//     type State = ();
//
//     fn play(
//         &self,
//         players: &mut Players<Move, (), N>,
//     ) -> PlayResult<Outcome<Move, Util, N>, Move, (), N> {
//         let profile = PerPlayer::generate(|i| players[i].next_move(&()));
//         for i in PlayerIndex::all_indexes() {
//             if !self.is_valid_move_for_player(i, profile[i]) {
//                 return Err(InvalidMove {
//                     player: i,
//                     the_move: profile[i],
//                     state: (),
//                 });
//             }
//         }
//         Ok(Outcome::new(profile, self.payoff(profile)))
//     }
// }
//
// impl<Move: IsMove, Util: IsUtil, const N: usize> Playable<N> for Normal<Move, Util, N> {
//     type Move = Move;
//     type Util = Util;
//     type Outcome = Outcome<Move, Util, N>;
//     type State = ();
//
//     fn play(
//         &self,
//         players: &mut Players<Move, (), N>,
//     ) -> PlayResult<Outcome<Move, Util, N>, Move, (), N> {
//         self.as_simultaneous().play(players)
//     }
// }
