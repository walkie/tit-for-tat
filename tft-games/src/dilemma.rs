//! 2x2 simultaneous social dilemma games.

use tft::core::{Payoff, PerPlayer};
use tft::game::Normal;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Dilemma { Cooperate, Defect }

pub const C: Dilemma = Dilemma::Cooperate;
pub const D: Dilemma = Dilemma::Defect;

/// Create a new social dilemma game with the given table of payoffs.
pub fn new(payoffs: [Payoff<i32, 2>; 4]) -> Normal<Dilemma, i32, 2> {
    Normal::new(
        PerPlayer::new([Vec::from([C, D]), Vec::from([C, D])]),
        Vec::from(payoffs),
    )
    .unwrap()
}
