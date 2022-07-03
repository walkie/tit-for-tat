//! This module defines the types related to pure strategy profiles for simultaneous games.

use either::Either;
use std::fmt::Debug;
use std::hash::Hash;
use std::iter::Iterator;

use crate::core::{PerPlayer, PlayerIndex};

/// A pure strategy profile for a simultaneous game: one move played by each player.
pub type Profile<Move, const N: usize> = PerPlayer<Move, N>;

/// A pattern describing the profiles to match and/or a profile iteration in progress.
type Pattern<Move, const N: usize> = PerPlayer<Either<Move, usize>, N>;

/// State of a profile iterator. Changes from `Init` to `Running` after `next()` is called once.
/// Changes from `Running` to `Exhausted` after the last element is returned.
enum State<Move, const N: usize> {
    /// Iterator in the initialization stage: before `next()` has been called. In this stage, the
    /// `including()` and `adjacent_to()` methods may be called. The pattern describes the first
    /// profile to return.
    Init(Pattern<Move, N>),
    /// Running iterator: after `next()` has been called once, but before the iterator is
    /// exhausted. The pattern describes the next profile to return.
    Running(Pattern<Move, N>),
    /// Exhausted iterator: there are no more profiles to return.
    Exhausted,
}

impl<Move, const N: usize> State<Move, N>
where
    Move: Copy + Debug + Eq + Hash,
{
    /// Construct a new initial state with an empty pattern.
    fn init() -> Self {
        State::Init(PerPlayer::new([Either::Right(0); N]))
    }
}

/// An iterator over all of the pure strategy profiles that can be generated from a list of moves
/// available to each player.
pub struct ProfileIter<Move, const N: usize> {
    moves: PerPlayer<Vec<Move>, N>,
    state: State<Move, N>,
}

impl<Move, const N: usize> ProfileIter<Move, N>
where
    Move: Copy + Debug + Eq + Hash,
{
    /// Construct a new profile iterator that iterates over all combinations of the provided
    /// vectors of available moves for each player.
    ///
    /// # Examples
    /// ```
    /// use tft::core::{PerPlayer, ProfileIter};
    ///
    /// let moves = PerPlayer::new([
    ///     vec!['A', 'B', 'C'],
    ///     vec!['D', 'E'],
    ///     vec!['F', 'G'],
    /// ]);
    /// let mut iter = ProfileIter::new(moves);
    /// assert_eq!(
    ///     iter.collect::<Vec<PerPlayer<char, 3>>>(),
    ///     vec![
    ///         PerPlayer::new(['A', 'D', 'F']), PerPlayer::new(['A', 'D', 'G']),
    ///         PerPlayer::new(['A', 'E', 'F']), PerPlayer::new(['A', 'E', 'G']),
    ///         PerPlayer::new(['B', 'D', 'F']), PerPlayer::new(['B', 'D', 'G']),
    ///         PerPlayer::new(['B', 'E', 'F']), PerPlayer::new(['B', 'E', 'G']),
    ///         PerPlayer::new(['C', 'D', 'F']), PerPlayer::new(['C', 'D', 'G']),
    ///         PerPlayer::new(['C', 'E', 'F']), PerPlayer::new(['C', 'E', 'G']),
    ///     ],
    /// );
    /// ```
    pub fn new(moves: PerPlayer<Vec<Move>, N>) -> Self {
        ProfileIter {
            moves,
            state: State::init(),
        }
    }

    /// Construct a new profile iterator for a game where each player has the same set of available
    /// moves.
    ///
    /// # Examples
    /// ```
    /// use tft::core::{PerPlayer, ProfileIter};
    ///
    /// let moves = PerPlayer::new([
    ///     vec!['A', 'B', 'C'],
    ///     vec!['D', 'E'],
    ///     vec!['F', 'G'],
    /// ]);
    /// let mut iter = ProfileIter::new(moves);
    /// assert_eq!(
    ///     iter.collect::<Vec<PerPlayer<char, 3>>>(),
    ///     vec![
    ///         PerPlayer::new(['A', 'D', 'F']), PerPlayer::new(['A', 'D', 'G']),
    ///         PerPlayer::new(['A', 'E', 'F']), PerPlayer::new(['A', 'E', 'G']),
    ///         PerPlayer::new(['B', 'D', 'F']), PerPlayer::new(['B', 'D', 'G']),
    ///         PerPlayer::new(['B', 'E', 'F']), PerPlayer::new(['B', 'E', 'G']),
    ///         PerPlayer::new(['C', 'D', 'F']), PerPlayer::new(['C', 'D', 'G']),
    ///         PerPlayer::new(['C', 'E', 'F']), PerPlayer::new(['C', 'E', 'G']),
    ///     ],
    /// );
    /// ```
    pub fn symmetric(moves: Vec<Move>) -> Self {
        ProfileIter {
            moves: PerPlayer::generate(|_| moves.clone()),
            state: State::init(),
        }
    }

    /// Constrain the iterator to generate only profiles that include the given move played by the
    /// given player.
    ///
    /// Multiple invocations of [`including()`](ProfileIter::including) and
    /// [`excluding()`](ProfileIter::excluding) can be chained together to add several constraints
    /// to the iterator.
    ///
    /// # Errors
    ///
    /// Logs an error and returns an exhausted iterator that returns no profiles if:
    /// - The provided move is not a valid move for the given player.
    /// - The iterator has already generated at least one profile.
    ///
    /// # Examples
    ///
    /// Constraining a single player's move:
    /// ```
    /// use tft::core::{for2, PerPlayer, ProfileIter};
    ///
    /// let moves = PerPlayer::new([
    ///     vec!['A', 'B'],
    ///     vec!['C', 'D', 'E'],
    /// ]);
    ///
    /// let mut iter = ProfileIter::new(moves.clone()).including(for2::P0, 'B');
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['B', 'C'])));
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['B', 'D'])));
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['B', 'E'])));
    /// assert_eq!(iter.next(), None);
    ///
    /// let mut iter = ProfileIter::new(moves).including(for2::P1, 'D');
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['A', 'D'])));
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['B', 'D'])));
    /// assert_eq!(iter.next(), None);
    /// ```
    ///
    /// Constraining multiple players' moves by chaining invocations of this method:
    /// ```
    /// use tft::core::{for3, PerPlayer, ProfileIter};
    ///
    /// let moves = PerPlayer::new([
    ///     vec!['A', 'B'],
    ///     vec!['C', 'D', 'E'],
    ///     vec!['F', 'G', 'H'],
    /// ]);
    ///
    /// let mut iter = ProfileIter::new(moves.clone())
    ///     .including(for3::P0, 'A')
    ///     .including(for3::P2, 'G');
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['A', 'C', 'G'])));
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['A', 'D', 'G'])));
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['A', 'E', 'G'])));
    /// assert_eq!(iter.next(), None);
    ///
    /// let mut iter = ProfileIter::new(moves)
    ///     .including(for3::P0, 'B')
    ///     .including(for3::P1, 'C');
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['B', 'C', 'F'])));
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['B', 'C', 'G'])));
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['B', 'C', 'H'])));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn including(self, player: PlayerIndex<N>, the_move: Move) -> Self {
        if let State::Init(mut pattern) = self.state {
            if self.moves[player].contains(&the_move) {
                pattern[player] = Either::Left(the_move);
                // success
                return ProfileIter {
                    moves: self.moves,
                    state: State::Init(pattern),
                };
            } else {
                log::error!(
                    "ProfileIter::including(): tried to constrain player {} to invalid move {:?}",
                    usize::from(player),
                    the_move
                );
            }
        } else {
            log::error!("ProfileIter::including(): tried to constrain an iterator that is already running or exhausted");
        }
        // error
        ProfileIter {
            moves: self.moves,
            state: State::Exhausted,
        }
    }

    /// Constrain the iterator to generate only those profiles that are "adjacent" to the given
    /// profile for the indicated player.
    ///
    /// An adjacent profile differs from the given profile only in the move of the given player.
    ///
    /// # Errors
    ///
    /// Logs an error and returns an exhausted iterator that returns no profiles if:
    /// - The provided profile contains a move that is invalid for the corresponding player.
    /// - The iterator has already generated at least one profile.
    ///
    /// # Examples
    /// ```
    /// use tft::core::{for3, PerPlayer, ProfileIter};
    ///
    /// let moves = PerPlayer::new([
    ///     vec!['A', 'B'],
    ///     vec!['C', 'D', 'E'],
    ///     vec!['F', 'G', 'H'],
    /// ]);
    /// let profile = PerPlayer::new(['A', 'D', 'H']);
    ///
    /// let mut iter = ProfileIter::new(moves.clone()).adjacent_to(for3::P0, profile);
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['B', 'D', 'H'])));
    /// assert_eq!(iter.next(), None);
    ///
    /// let mut iter = ProfileIter::new(moves.clone()).adjacent_to(for3::P1, profile);
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['A', 'C', 'H'])));
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['A', 'E', 'H'])));
    /// assert_eq!(iter.next(), None);
    ///
    /// let mut iter = ProfileIter::new(moves).adjacent_to(for3::P2, profile);
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['A', 'D', 'F'])));
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['A', 'D', 'G'])));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn adjacent_to(self, player: PlayerIndex<N>, profile: Profile<Move, N>) -> Self {
        // check that the profile contains only valid moves
        if PlayerIndex::all_indexes().all(|i| self.moves[i].contains(&profile[i])) {
            if let State::Init(old_pattern) = self.state {
                let new_pattern = PerPlayer::generate(|i| {
                    if i == player {
                        old_pattern[i]
                    } else {
                        Either::Left(profile[i])
                    }
                });
                // success
                return ProfileIter {
                    moves: self.moves,
                    state: State::Init(new_pattern),
                };
            } else {
                log::error!("ProfileIter::adjacent_to(): tried to constrain an iterator that is already running or exhausted");
            }
        } else {
            log::error!("ProfileIter::adjacent_to(): profile contains invalid moves: {:?}", profile);
        }
        // error
        ProfileIter {
            moves: self.moves,
            state: State::Exhausted,
        }
    }
}

impl<Move, const N: usize> Iterator for ProfileIter<Move, N>
where
    Move: Copy + Debug + Eq + Hash,
{
    type Item = Profile<Move, N>;

    fn next(&mut self) -> Option<Profile<Move, N>> {
        if let State::Init(pattern) = self.state {
            // this is the first time we've called next, switch to "running" state
            self.state = State::Running(pattern);
        }

        if let State::Running(mut pattern) = self.state {
            // turn pattern into profile
            let profile = PerPlayer::generate(|player| match pattern[player] {
                Either::Left(m) => m,
                Either::Right(i) => self.moves[player][i],
            });

            // try to increment pattern
            let mut has_next = false;
            for player in PlayerIndex::all_indexes().rev() {
                match pattern[player] {
                    Either::Left(_) => {}
                    Either::Right(i) => {
                        if i + 1 < self.moves[player].len() {
                            has_next = true;
                            pattern[player] = Either::Right(i + 1);
                            self.state = State::Running(pattern);
                            break;
                        } else {
                            pattern[player] = Either::Right(0);
                        }
                    }
                }
            }

            // if incrementing failed, set the state to "exhausted"
            if !has_next {
                self.state = State::Exhausted;
            }

            // return profile
            Some(profile)
        } else {
            // no more profiles
            None
        }
    }
}
