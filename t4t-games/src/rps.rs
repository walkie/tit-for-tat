//! Rock-paper-scissors and related games.

use t4t::prelude::norm::*;

/// A move in rock-paper-scissors-style game.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
    Fire,
    Water,
}

/// The classic [rock-paper-scissors](https://en.wikipedia.org/wiki/Rock_paper_scissors) game.
///
/// The moves are related as follows:
///
/// - `Rock` beats `Scissors`
/// - `Scissors` beats `Paper`
/// - `Paper` beats `Rock`
///
/// A winning move is awarded `1`, a losing move is awarded `-1`, and ties are awarded `0`.
///
/// # Examples
/// ```
/// use t4t::prelude::norm::*;
/// use t4t_games::rps;
///
/// let rps = rps::rock_paper_scissors();
/// assert!(rps.is_zero_sum());
/// ```
#[rustfmt::skip]
pub fn rock_paper_scissors() -> Normal<Move, i64, 2> {
    Normal::symmetric(
        vec![Move::Rock, Move::Paper, Move::Scissors],
        vec![ 0, -1,  1,
              1,  0, -1,
             -1,  1,  0,
        ],
    ).unwrap()
}

/// Rock-paper-scissors extended with two news moves: fire and water.
///
/// In this version of the game, `Rock`, `Paper`, and `Scissors` are related to each other as in
/// the basic game. Additionally:
///
/// - `Fire` beats `Rock`, `Paper`, and `Scissors`, but loses to `Water`
/// - `Water` beats `Fire`, but loses to `Rock`, `Paper`, and `Scissors`
///
/// # Examples
/// ```
/// use t4t::prelude::norm::*;
/// use t4t_games::rps;
///
/// let fw = rps::fire_water();
/// assert!(fw.is_zero_sum());
/// ```
#[rustfmt::skip]
pub fn fire_water() -> Normal<Move, i64, 2> {
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

/// An N-player version of rock-paper-scissors.
///
/// Each player is assigned a utility value that is computed by counting the number of players that
/// the player beat subtracted by the number of players that beat the player. For example, playing
/// `Rock` will give a utility value equal to the number of `Scissors` moves played by other
/// players minus the number of `Paper` moves played by other players.
///
/// # Examples
///
/// ```
/// use t4t::prelude::norm::*;
/// use t4t_games::rps;
///
/// // 10-player rock-paper-scissors.
/// let rps10: Normal<rps::Move, i64, 10> = rps::big_rock_paper_scissors();
/// assert!(rps10.is_zero_sum());
///
/// // 1000-player rock-paper-scissors!
/// let rps1000: Normal<rps::Move, i64, 1000> = rps::big_rock_paper_scissors();
/// ```
///
/// Note that `rps1000` demonstrates that `Normal` can represent extremely large games---this game
/// has a payoff table with `3^1000` entries! Such large games can be represented and played
/// without issue, but any function that iterates over the outcomes (such as
/// [`is_zero_sum`](t4t::Normal::is_zero_sum) or any solution concept), will leave you waiting
/// beyond the heat death of the universe.
#[rustfmt::skip]
pub fn big_rock_paper_scissors<const N: usize>() -> Normal<Move, i64, N> {
    let moves = PerPlayer::init_with(vec![Move::Rock, Move::Paper, Move::Scissors]);
    let payoff_fn = |profile: Profile<Move, N>| {
        let mut rocks = 0;
        let mut papers = 0;
        let mut scissors = 0;
        for m in profile {
            match m {
                Move::Rock => rocks += 1,
                Move::Paper => papers += 1,
                Move::Scissors => scissors += 1,
                _ => log::warn!("Unexpected move: {:?}", m),
            }
        }
        let rock_util = scissors - papers;
        let paper_util = rocks - scissors;
        let scissors_util = papers - rocks;
        Payoff::new(profile.map(|m|
            match m {
                Move::Rock => rock_util,
                Move::Paper => paper_util,
                Move::Scissors => scissors_util,
                _ => 0,
            }
        ))
    };
    Normal::from_payoff_fn(moves, payoff_fn)
}
