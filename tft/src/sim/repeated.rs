use crate::moves::IsMove;
use crate::outcome::*;
use crate::payoff::{IsUtility, Payoff};
use crate::play::*;
use crate::simultaneous::Simultaneous;
use crate::transcript::Transcript;

/// For repeated games, a record of previously played games.
#[derive(Clone)]
pub struct History<Outcome, Util, const N: usize> {
    played: Vec<Outcome>,
    score: Payoff<Util, N>,
}

/// For repeated sequential games, a record of previously played games.
pub type HistorySeq<Move, Util, const N: usize> = History<OutcomeSeq<Move, Util, N>, Util, N>;

/// For repeated simultaneous games, a record of previously played games.
pub type HistorySim<Move, Util, const N: usize> = History<OutcomeSim<Move, Util, N>, Util, N>;


#[derive(Clone)]
pub struct IteratedState<State, Outcome, Util, const N: usize> {
    game_state: State,
    history: History<Outcome, Util, N>,
}

pub type IteratedStateSeq<State, Move, Util, const N: usize> =
    IteratedState<State, OutcomeSeq<Move, Util, N>, Util, N>;

pub type IteratedStateSim<State, Move, Util, const N: usize> =
    IteratedState<State, OutcomeSeq<Move, Util, N>, Util, N>;


impl<State, Util, Outcome, const N: usize> HasGameState<State> for IteratedState<State, Util, Outcome, N> {
    fn game_state(&self) -> &State {
        &self.game_state
    }
}

impl<Util: IsUtility, Outcome, const N: usize> Default for History<Util, Outcome, N> {
    fn default() -> Self {
        History {
            played: Vec::new(),
            score: Payoff::zeros(),
        }
    }
}

impl<Move: IsMove, Util: IsUtility, Outcome: IsOutcome<Move, Util, N>, const N: usize>
    IsOutcome<Move, Util, N> for History<Util, Outcome, N>
{
    fn transcript(&self) -> Transcript<Move, N> {
        Transcript::from_played_moves(
            self.played
                .iter()
                .flat_map(|outcome| outcome.transcript())
                .collect(),
        )
    }

    fn payoff(&self) -> Payoff<Util, N> {
        self.score
    }
}

impl<Game, Move: IsMove, Util: IsUtility, const N: usize> Sequentially<N> for Box<Game>
where
    Game: Simultaneously<N, Move = Move, Util = Util>,
{
    type Move = Move;
    type Util = Util;
    type State = IteratedState<(), Util, SimultaneousOutcome<Move, Util, N>, N>;

    fn play<::State>(&self, players: &mut Players<Move, Self::State, N>) -> SimultaneousResult<Move, Util, N> {
        unimplemented!()
    }
}

// impl<Outcome: HasPayoff<Util, N>, Util: IsUtil, const N: usize> History<Outcome, Util, N> {
//     pub fn new() -> Self {
//         History::default()
//     }
//
//     pub fn add(&mut self, played: Outcome) {
//         self.score = self.score + played.payoff();
//         self.played.push(played);
//     }
//
//     pub fn score(&self) -> Payoff<Util, N> {
//         self.score
//     }
// }
//
// pub struct IteratedState<Game: Playable<N>, const N: usize> {
//     history: History<Game::Outcome, Game::Util, N>,
//     game_state: Game::State,
// }
//
// pub struct Iterated<Game> {
//     game: Game,
// }
//
// impl<Move: IsMove, Util: IsUtil, const N: usize> Playable<N>
//     for Iterated<Simultaneous<Move, Util, N>>
// {
//     type Move = Move;
//     type Util = Util;
//     type Outcome = History<Outcome<Move, Util, N>, Util, N>;
//     type State = History<Outcome<Move, Util, N>, Util, N>;
//     fn play(
//         &self,
//         players: &mut Players<Move, Self::State, N>,
//     ) -> PlayResult<Self::Outcome, Move, Self::State, N> {
//         unimplemented!()
//     }
// }
