//! Rock-paper-scissors and related games.

use tft::norm::Normal;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
    Fire,
    Water,
}

/// Classic [rock-paper-scissors](https://en.wikipedia.org/wiki/Rock_paper_scissors) game:
/// - `Rock` beats `Scissors`
/// - `Scissors` beats `Paper`
/// - `Paper` beats `Rock`
///
/// # Examples
/// ```
/// use tft::norm::*;
/// use tft_games::rps;
///
/// let rps = rps::rock_paper_scissors();
/// assert!(rps.is_zero_sum());
/// ```
#[rustfmt::skip]
pub fn rock_paper_scissors() -> Normal<Move, i8, 2> {
    Normal::symmetric(
        vec![Move::Rock, Move::Paper, Move::Scissors],
        vec![ 0, -1,  1,
              1,  0, -1,
             -1,  1,  0,
        ],
    ).unwrap()
}

/// In extended rock-paper-scissors, `Rock`, `Paper`, and `Scissors` are related to each other as
/// in the basic game. Additionally:
/// - `Fire` beats `Rock`, `Paper`, and `Scissors`, but loses to `Water`
/// - `Water` beats `Fire`, but loses to `Rock`, `Paper`, and `Scissors`
///
/// # Examples
/// ```
/// use tft::norm::*;
/// use tft_games::rps;
///
/// let fw = rps::fire_water();
/// assert!(fw.is_zero_sum());
/// ```
#[rustfmt::skip]
pub fn fire_water() -> Normal<Move, i8, 2> {
    Normal::symmetric(
        vec![Move::Rock, Move::Paper, Move::Scissors, Move::Fire, Move::Water],
        vec![ 0, -1,  1, -1,  1,
              1,  0, -1, -1,  1,
             -1,  1,  0, -1,  1,
              1,  1,  1,  0, -1,
             -1, -1, -1,  1,  0,
        ],
    ).unwrap()
}
