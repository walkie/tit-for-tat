//! This module defines the types related to pure strategy profiles for simultaneous games.

use itertools::structs::MultiProduct;
use itertools::Itertools;
use std::fmt::Debug;
use std::hash::Hash;
use std::iter::Iterator;

use crate::core::{PerPlayer, PlayerIndex};

/// A pure strategy profile for a simultaneous game: one move played by each player.
pub type Profile<Move, const N: usize> = PerPlayer<Move, N>;

/// An iterator over all of the pure strategy profiles that can be generated from the moves
/// available to each player.
#[derive(Clone, Debug)]
pub struct ProfileIter<Move: Copy, MoveIter: Iterator<Item = Move> + Clone, const N: usize> {
    /// Moves that must be included in any generated profile, for each player.
    includes: PerPlayer<Vec<Move>, N>,
    /// Moves that must be excluded from any generated profile, for each player.
    excludes: PerPlayer<Vec<Move>, N>,
    /// The multi-product iterator used to generate each profile.
    multi_iter: MultiProduct<MoveIter>,
}

impl<Move, MoveIter, const N: usize> ProfileIter<Move, MoveIter, N>
where
    Move: Copy + Debug + Eq + Hash,
    MoveIter: Iterator<Item = Move> + Clone,
{
    /// Construct a new profile iterator from a per-player collection of iterators over the moves
    /// available to each player.
    ///
    /// # Examples
    /// ```
    /// use tft::core::{PerPlayer, ProfileIter};
    ///
    /// let moves = PerPlayer::new([
    ///     vec!['A', 'B', 'C'].into_iter(),
    ///     vec!['D', 'E'].into_iter(),
    ///     vec!['F', 'G'].into_iter(),
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
    pub fn new(move_iters: PerPlayer<MoveIter, N>) -> Self {
        ProfileIter {
            includes: PerPlayer::init_with(Vec::new()),
            excludes: PerPlayer::init_with(Vec::new()),
            multi_iter: move_iters.into_iter().multi_cartesian_product(),
        }
    }

    /// Construct a new profile iterator for a game where each player has the same set of available
    /// moves.
    ///
    /// # Examples
    ///
    /// Generating all profiles for a symmetric 2-player game:
    /// ```
    /// use tft::core::{PerPlayer, ProfileIter};
    ///
    /// let mut iter = ProfileIter::symmetric(vec!['X', 'O'].into_iter());
    /// assert_eq!(
    ///     iter.collect::<Vec<PerPlayer<char, 2>>>(),
    ///     vec![
    ///         PerPlayer::new(['X', 'X']), PerPlayer::new(['X', 'O']),
    ///         PerPlayer::new(['O', 'X']), PerPlayer::new(['O', 'O']),
    ///     ],
    /// );
    ///
    /// let mut iter = ProfileIter::symmetric(vec!['A', 'B', 'C'].into_iter());
    /// assert_eq!(
    ///     iter.collect::<Vec<PerPlayer<char, 2>>>(),
    ///     vec![
    ///         PerPlayer::new(['A', 'A']), PerPlayer::new(['A', 'B']), PerPlayer::new(['A', 'C']),
    ///         PerPlayer::new(['B', 'A']), PerPlayer::new(['B', 'B']), PerPlayer::new(['B', 'C']),
    ///         PerPlayer::new(['C', 'A']), PerPlayer::new(['C', 'B']), PerPlayer::new(['C', 'C']),
    ///     ],
    /// );
    /// ```
    ///
    /// Generating all profiles for a symmetric 3-player game:
    /// ```
    /// use tft::core::{PerPlayer, ProfileIter};
    ///
    /// let mut iter = ProfileIter::symmetric(vec!['X', 'O'].into_iter());
    /// assert_eq!(
    ///     iter.collect::<Vec<PerPlayer<char, 3>>>(),
    ///     vec![
    ///         PerPlayer::new(['X', 'X', 'X']), PerPlayer::new(['X', 'X', 'O']),
    ///         PerPlayer::new(['X', 'O', 'X']), PerPlayer::new(['X', 'O', 'O']),
    ///         PerPlayer::new(['O', 'X', 'X']), PerPlayer::new(['O', 'X', 'O']),
    ///         PerPlayer::new(['O', 'O', 'X']), PerPlayer::new(['O', 'O', 'O']),
    ///     ],
    /// );
    /// ```
    pub fn symmetric(move_iter: MoveIter) -> Self {
        ProfileIter::new(PerPlayer::init_with(move_iter))
    }

    /// Constrain the iterator to generate only profiles where the given player plays a specific
    /// move.
    ///
    /// If the move is not a valid move for that player, then the resulting iterator will not
    /// generate any profiles.
    ///
    /// Multiple invocations of [`include()`](ProfileIter::include) and
    /// [`exclude()`](ProfileIter::exclude) can be chained together to add several constraints to
    /// the iterator.
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
    /// let mut iter = ProfileIter::from_vecs(moves.clone()).include(for2::P0, 'B');
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['B', 'C'])));
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['B', 'D'])));
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['B', 'E'])));
    /// assert_eq!(iter.next(), None);
    ///
    /// let mut iter = ProfileIter::from_vecs(moves).include(for2::P1, 'D');
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['A', 'D'])));
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['B', 'D'])));
    /// assert_eq!(iter.next(), None);
    /// ```
    ///
    /// Constraining multiple players' moves by chaining invocations of this method:
    /// ```
    /// use tft::core::{for3, PerPlayer, ProfileIter};
    ///
    /// let mut iter = ProfileIter::symmetric(vec!['A', 'B', 'C'].into_iter())
    ///     .include(for3::P0, 'A')
    ///     .include(for3::P2, 'C');
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['A', 'A', 'C'])));
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['A', 'B', 'C'])));
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['A', 'C', 'C'])));
    /// assert_eq!(iter.next(), None);
    /// ```
    ///
    /// Combining with [`exclude()`](ProfileIter::exclude):
    /// ```
    /// use tft::core::{for3, PerPlayer, ProfileIter};
    ///
    /// let moves = PerPlayer::new([
    ///     vec!['A', 'B'],
    ///     vec!['C', 'D', 'E'],
    ///     vec!['F', 'G', 'H'],
    /// ]);
    ///
    /// let mut iter = ProfileIter::from_vecs(moves.clone())
    ///     .include(for3::P1, 'D')
    ///     .exclude(for3::P2, 'G');
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['A', 'D', 'F'])));
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['A', 'D', 'H'])));
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['B', 'D', 'F'])));
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['B', 'D', 'H'])));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn include(self, player: PlayerIndex<N>, the_move: Move) -> Self {
        let mut includes = self.includes;
        includes[player].push(the_move);
        ProfileIter { includes, ..self }
    }

    /// Constrain the iterator to generate only profiles where the given player *does not* play a
    /// specific move.
    ///
    /// If the move is not a valid move for that player, then this method will have no effect.
    ///
    /// Multiple invocations of [`include()`](ProfileIter::include) and
    /// [`exclude()`](ProfileIter::exclude) can be chained together to add several constraints to
    /// the iterator.
    ///
    /// # Examples
    ///
    /// Applying a single exlcusion constraint:
    /// ```
    /// use tft::core::{for2, PerPlayer, ProfileIter};
    ///
    /// let moves = PerPlayer::new([
    ///     vec!['A', 'B'],
    ///     vec!['C', 'D', 'E'],
    /// ]);
    ///
    /// let mut iter = ProfileIter::from_vecs(moves).exclude(for2::P1, 'C');
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['A', 'D'])));
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['A', 'E'])));
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['B', 'D'])));
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['B', 'E'])));
    /// assert_eq!(iter.next(), None);
    /// ```
    ///
    /// Applying multiple exclusion constraints by chaining invocations of this method:
    /// ```
    /// use tft::core::{for2, PerPlayer, ProfileIter};
    ///
    /// let moves = PerPlayer::new([
    ///     vec!['A', 'B', 'C'],
    ///     vec!['D', 'E', 'F', 'G'],
    /// ]);
    ///
    /// let mut iter = ProfileIter::from_vecs(moves)
    ///     .exclude(for2::P0, 'A')
    ///     .exclude(for2::P1, 'E')
    ///     .exclude(for2::P1, 'G');
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['B', 'D'])));
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['B', 'F'])));
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['C', 'D'])));
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['C', 'F'])));
    /// assert_eq!(iter.next(), None);
    /// ```
    ///
    /// Combining with [`include()`](ProfileIter::include):
    /// ```
    /// use tft::core::{for3, PerPlayer, ProfileIter};
    ///
    /// let mut iter = ProfileIter::symmetric(vec!['A', 'B', 'C'].into_iter())
    ///     .exclude(for3::P0, 'A')
    ///     .exclude(for3::P1, 'B')
    ///     .include(for3::P2, 'C');
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['B', 'A', 'C'])));
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['B', 'C', 'C'])));
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['C', 'A', 'C'])));
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['C', 'C', 'C'])));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn exclude(self, player: PlayerIndex<N>, the_move: Move) -> Self {
        let mut excludes = self.excludes;
        excludes[player].push(the_move);
        ProfileIter { excludes, ..self }
    }

    /// Constrain the iterator to generate only profiles that are "adjacent" to the given profile
    /// for a given player.
    ///
    /// An adjacent profile is one where the given player plays a different move, but all other
    /// players play the move specified in the profile.
    ///
    /// # Examples
    /// ```
    /// use tft::core::{for5, PerPlayer, ProfileIter};
    ///
    /// let mut iter = ProfileIter::symmetric(vec![1, 2, 3, 4, 5].into_iter())
    ///     .adjacent(for5::P3, PerPlayer::new([5, 4, 3, 2, 1]));
    /// assert_eq!(iter.next(), Some(PerPlayer::new([5, 4, 3, 1, 1])));
    /// assert_eq!(iter.next(), Some(PerPlayer::new([5, 4, 3, 3, 1])));
    /// assert_eq!(iter.next(), Some(PerPlayer::new([5, 4, 3, 4, 1])));
    /// assert_eq!(iter.next(), Some(PerPlayer::new([5, 4, 3, 5, 1])));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn adjacent(self, player: PlayerIndex<N>, profile: Profile<Move, N>) -> Self {
        let mut iter = self;
        for i in PlayerIndex::all_indexes() {
            if i == player {
                iter = iter.exclude(i, profile[i]);
            } else {
                iter = iter.include(i, profile[i]);
            }
        }
        iter
    }
}

impl<Move, const N: usize> ProfileIter<Move, std::vec::IntoIter<Move>, N>
where
    Move: Copy + Debug + Eq + Hash,
{
    /// Construct a new profile iterator from a per-player collection of vectors of available moves
    /// for each player.
    ///
    /// # Examples
    /// ```
    /// use tft::core::{for2, PerPlayer, ProfileIter};
    ///
    /// let moves = PerPlayer::new([
    ///     vec!['A', 'B', 'C', 'D'],
    ///     vec!['E', 'F', 'G'],
    /// ]);
    /// let mut iter = ProfileIter::from_vecs(moves).include(for2::P1, 'F');
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['A', 'F'])));
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['B', 'F'])));
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['C', 'F'])));
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['D', 'F'])));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn from_vecs(move_vecs: PerPlayer<Vec<Move>, N>) -> Self {
        ProfileIter::new(move_vecs.map(|v| v.into_iter()))
    }
}

impl<Move, MoveIter, const N: usize> Iterator for ProfileIter<Move, MoveIter, N>
where
    Move: Copy + Debug + Eq + Hash,
    MoveIter: Iterator<Item = Move> + Clone,
{
    type Item = Profile<Move, N>;

    fn next(&mut self) -> Option<Profile<Move, N>> {
        for moves in self.multi_iter.by_ref() {
            let profile = PerPlayer::new(moves.try_into().unwrap());
            let mut good = true;
            for player in PlayerIndex::all_indexes() {
                let m = profile[player];
                if self.excludes[player].contains(&m)
                    || !self.includes[player].is_empty() && !self.includes[player].contains(&m)
                {
                    good = false;
                    break;
                }
            }
            if good {
                return Some(profile);
            }
        }
        None
    }
}
