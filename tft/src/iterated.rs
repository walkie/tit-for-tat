use crate::moves::IsMove;
use crate::outcome::Outcome;
use crate::payoff::{IsUtil, Payoff};
use crate::play::*;
use crate::player::Players;
use crate::simultaneous::Simultaneous;

/// For iterated games, a record of previously played games.
#[derive(Clone)]
pub struct History<Outcome, Util, const N: usize> {
    played: Vec<Outcome>,
    score: Payoff<Util, N>,
}

impl<Outcome, Util: IsUtil, const N: usize> Default for History<Outcome, Util, N> {
    fn default() -> Self {
        History {
            played: Vec::new(),
            score: Payoff::zeros(),
        }
    }
}

impl<Outcome, Util: IsUtil, const N: usize> HasPayoff<Util, N> for History<Outcome, Util, N> {
    fn payoff(&self) -> Payoff<Util, N> {
        self.score
    }
}

impl<Outcome: HasPayoff<Util, N>, Util: IsUtil, const N: usize> History<Outcome, Util, N> {
    pub fn new() -> Self {
        History::default()
    }

    pub fn add(&mut self, played: Outcome) {
        self.score = self.score + played.payoff();
        self.played.push(played);
    }

    pub fn score(&self) -> Payoff<Util, N> {
        self.score
    }
}

pub struct IteratedState<Game: Playable<N>, const N: usize> {
    // history: History<<Game as Playable<N>>::Outcome, <Game as Playable<N>>::Util, N>,
    history: History<Game::Outcome, Game::Util, N>,
    game_state: Game::State,
}

pub struct Iterated<Game> {
    game: Game,
}

impl<Move: IsMove, Util: IsUtil, const N: usize> Playable<N>
    for Iterated<Simultaneous<Move, Util, N>>
{
    type Move = Move;
    type Util = Util;
    type Outcome = History<Outcome<Move, Util, N>, Util, N>;
    type State = History<Outcome<Move, Util, N>, Util, N>;
    fn play(
        &self,
        players: &mut Players<Move, Self::State, N>,
    ) -> PlayResult<Self::Outcome, Move, Self::State, N> {
        unimplemented!()
    }
}
