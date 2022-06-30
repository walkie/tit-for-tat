//! A lookup table of payoffs. Used for normal-form game representations.

use either::Either;
use itertools::Itertools;
use num::Num;
// use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::iter::Iterator;

use crate::core::{Payoff, PerPlayer, PlayerIndex};
use crate::game::Profile;

/// A lookup table of payoffs indexed by the move played by each player. Used for normal-form game
/// representations.
///
/// # Type variables
/// - `Move` -- The type of moves played during the game.
/// - `Util` -- The type of utility value awarded to each player in a payoff.
/// - `N` -- The number of players that play the game.
///
/// # Examples
pub struct Table<Move, Util, const N: usize> {
    moves: PerPlayer<Vec<Move>, N>,
    payoff_map: HashMap<Profile<Move, N>, Payoff<Util, N>>,
}

impl<Move, Util, const N: usize> Table<Move, Util, N>
where
    Move: Copy + Debug + Eq + Hash,
    Util: Copy + Debug + Num + Ord,
{
    /// Construct a payoff table given the moves available to each player and a vector of payoffs
    /// in [row-major order](https://en.wikipedia.org/wiki/Row-_and_column-major_order).
    ///
    /// # Errors
    /// Logs a warning and returns `None` if the number of provided payoffs is fewer than the
    /// number of unique profiles that can be generated from the available moves. If too many
    /// payoffs are provided, the excess payoffs will be ignored.
    ///
    /// # Examples
    pub fn new(moves: PerPlayer<Vec<Move>, N>, payoffs: Vec<Payoff<Util, N>>) -> Option<Self> {
        let profiles: Vec<PerPlayer<Move, N>> = moves
            .clone()
            .into_iter()
            .multi_cartesian_product()
            .map(|vec| PerPlayer::new(vec.try_into().unwrap()))
            .collect();

        if profiles.len() > payoffs.len() {
            log::warn!(
                "Normal::new(): expected a vector of {} payoffs, got only {}",
                profiles.len(),
                payoffs.len()
            );
            return None;
        }

        let mut payoff_map = HashMap::with_capacity(profiles.len());
        for (profile, payoff) in profiles.into_iter().zip(payoffs) {
            payoff_map.insert(profile, payoff);
        }
        Some(Table { moves, payoff_map })
    }

    pub fn moves(&self) -> &PerPlayer<Vec<Move>, N> {
        &self.moves
    }

    pub fn payoff(&self, profile: Profile<Move, N>) -> Option<Payoff<Util, N>> {
        self.payoff_map.get(&profile).copied()
    }

    pub fn cells(&self) -> CellIter<Move, Util, N> {
        CellIter::new(self)
    }

    pub fn payoffs(&self) -> PayoffIter<Move, Util, N> {
        PayoffIter::new(self)
    }

    pub fn profiles(&self) -> ProfileIter<Move, Util, N> {
        ProfileIter::new(self)
    }
}

impl<'t, Move, Util, const N: usize> IntoIterator for &'t Table<Move, Util, N>
where
    Move: Copy + Debug + Eq + Hash,
    Util: Copy + Debug + Num + Ord,
{
    type Item = TableCell<Move, Util, N>;
    type IntoIter = CellIter<'t, Move, Util, N>;
    fn into_iter(self) -> Self::IntoIter {
        self.cells()
    }
}

/// An iterator over profiles in a payoff table.
///
/// # Examples
///
/// By default, this iterates over every corresponding profile and payoff in the table:
/// ```
/// ```
///
/// However, moves can be restricted...
pub struct ProfileIter<'t, Move, Util, const N: usize> {
    table: &'t Table<Move, Util, N>,
    next_pattern: Option<PerPlayer<Either<Move, usize>, N>>,
}

impl<'t, Move, Util, const N: usize> ProfileIter<'t, Move, Util, N>
where
    Move: Copy + Debug + Eq + Hash,
    Util: Copy + Debug + Num + Ord,
{
    pub fn new(table: &'t Table<Move, Util, N>) -> Self {
        ProfileIter {
            table,
            next_pattern: Some(PerPlayer::new([Either::Right(0); N])),
        }
    }

    pub fn such_that(mut self, player: PlayerIndex<N>, the_move: Move) -> Self {
        if let Some(mut pattern) = self.next_pattern {
            if self.table.moves()[player].contains(&the_move) {
                pattern[player] = Either::Left(the_move);
                self.next_pattern = Some(pattern);
            } else {
                log::error!(
                    "TableIter::such_that(): tried to set player {} to an invalid move {:?}",
                    usize::from(player),
                    the_move
                );
                self.next_pattern = None;
            }
        } else {
            log::error!("TableIter::such_that(): tried to set a move in an exhausted iterator");
        }
        self
    }
}

impl<'t, Move, Util, const N: usize> Iterator for ProfileIter<'t, Move, Util, N>
where
    Move: Copy + Debug + Eq + Hash,
    Util: Copy + Debug + Num + Ord,
{
    type Item = Profile<Move, N>;

    fn next(&mut self) -> Option<Profile<Move, N>> {
        if let Some(mut pattern) = self.next_pattern {
            // turn pattern into profile
            let profile = PerPlayer::generate(|player| match pattern[player] {
                Either::Left(m) => m,
                Either::Right(i) => self.table.moves[player][i],
            });

            // try to increment next_pattern
            let mut has_next = false;
            for player in PlayerIndex::all_indexes().rev() {
                match pattern[player] {
                    Either::Left(_) => {}
                    Either::Right(i) => {
                        if i + 1 < self.table.moves[player].len() {
                            has_next = true;
                            pattern[player] = Either::Right(i + 1);
                            self.next_pattern = Some(pattern);
                            break;
                        } else {
                            pattern[player] = Either::Right(0);
                        }
                    }
                }
            }

            // if incrementing failed, set next_pattern to `None`
            if !has_next {
                self.next_pattern = None;
            }

            // return profile
            Some(profile)
        } else {
            // no more profiles
            None
        }
    }
}

pub struct PayoffIter<'t, Move, Util, const N: usize> {
    profile_iter: ProfileIter<'t, Move, Util, N>
}

impl<'t, Move, Util, const N: usize> PayoffIter<'t, Move, Util, N>
where
    Move: Copy + Debug + Eq + Hash,
    Util: Copy + Debug + Num + Ord,
{
    pub fn new(table: &'t Table<Move, Util, N>) -> Self {
        PayoffIter {
            profile_iter: ProfileIter::new(table)
        }
    }
}

impl<'t, Move, Util, const N: usize> Iterator for PayoffIter<'t, Move, Util, N>
where
    Move: Copy + Debug + Eq + Hash,
    Util: Copy + Debug + Num + Ord,
{
    type Item = Payoff<Util, N>;
    fn next(&mut self) -> Option<Self::Item> {
        self.profile_iter.next().map(|profile| self.profile_iter.table.payoff(profile).unwrap())
    }
}

/// A cell in a payoff table. The `profile` is the address of the cell while the `payoff` is its
/// value.
pub struct TableCell<Move, Util, const N: usize> {
    pub profile: Profile<Move, N>,
    pub payoff: Payoff<Util, N>,
}

pub struct CellIter<'t, Move, Util, const N: usize> {
    profile_iter: ProfileIter<'t, Move, Util, N>
}

impl<'t, Move, Util, const N: usize> CellIter<'t, Move, Util, N>
where
    Move: Copy + Debug + Eq + Hash,
    Util: Copy + Debug + Num + Ord,
{
    pub fn new(table: &'t Table<Move, Util, N>) -> Self {
        CellIter {
            profile_iter: ProfileIter::new(table)
        }
    }
}

impl<'t, Move, Util, const N: usize> Iterator for CellIter<'t, Move, Util, N>
where
    Move: Copy + Debug + Eq + Hash,
    Util: Copy + Debug + Num + Ord,
{
    type Item = TableCell<Move, Util, N>;
    fn next(&mut self) -> Option<Self::Item> {
        self.profile_iter.next().map(|profile| TableCell {
            profile,
            payoff: self.profile_iter.table.payoff(profile).unwrap(),
        })
    }
}
