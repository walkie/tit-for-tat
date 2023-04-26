use std::fmt::Debug;

use crate::game::Game;
use crate::payoff::Payoff;

/// Record of a completed game.
#[derive(Debug, PartialEq)]
pub struct GameRecord<G: Game<P>, const P: usize> {
    moves: G::MoveRecord,
    payoff: Payoff<G::Utility, P>,
}

impl<G: Game<P>, const P: usize> Clone for GameRecord<G, P> {
    fn clone(&self) -> Self {
        GameRecord {
            moves: self.moves.clone(),
            payoff: self.payoff,
        }
    }
}

/// For repeated games, a record of previously played games.
#[derive(Clone, Debug, PartialEq)]
pub struct History<G: Game<P>, const P: usize> {
    records: Vec<GameRecord<G, P>>,
    score: Payoff<G::Utility, P>,
}

impl<G: Game<P>, const P: usize> GameRecord<G, P> {
    /// Construct a new game record.
    pub fn new(moves: G::MoveRecord, payoff: Payoff<G::Utility, P>) -> Self {
        GameRecord { moves, payoff }
    }

    /// Get the moves played during this game.
    pub fn moves(&self) -> &G::MoveRecord {
        &self.moves
    }

    /// Get the payoff awarded at the end of this game.
    pub fn payoff(&self) -> Payoff<G::Utility, P> {
        self.payoff
    }
}

impl<G: Game<P>, const P: usize> Default for History<G, P> {
    fn default() -> Self {
        History {
            records: Vec::new(),
            score: Payoff::zeros(),
        }
    }
}

impl<G: Game<P>, const P: usize> History<G, P> {
    /// Construct a new, empty history.
    pub fn new() -> Self {
        History::default()
    }

    /// Update the history by adding a new completed game record. Returns a reference to the newly
    /// created record.
    pub fn add(
        &mut self,
        moves: G::MoveRecord,
        payoff: Payoff<G::Utility, P>,
    ) -> &GameRecord<G, P> {
        self.score = self.score + payoff;
        self.records.push(GameRecord::new(moves, payoff));
        self.records.last().unwrap()
    }

    /// Get the current score of the game.
    pub fn score(&self) -> Payoff<G::Utility, P> {
        self.score
    }
}
