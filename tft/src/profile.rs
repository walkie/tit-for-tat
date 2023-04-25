//! This module defines the types related to pure strategy profiles for simultaneous games.

use dyn_clone::DynClone;
use itertools::structs::MultiProduct;
use itertools::Itertools;
use std::iter::Iterator;

use crate::game::Move;
use crate::per_player::{PerPlayer, PlayerIndex};
use crate::transcript::{Played, Transcript};

/// A pure strategy profile for a simultaneous game: one move played by each player.
pub type Profile<M, const P: usize> = PerPlayer<M, P>;

impl<M: Move, const P: usize> Profile<M, P> {
    /// Attempt to construct a profile from the given transcript.
    ///
    /// Returns `None` if the transcript does not contain exactly one move per player.
    pub fn from_transcript(transcript: Transcript<M, P>) -> Option<Self> {
        transcript.to_profile()
    }

    /// Convert this profile into a transcript.
    pub fn to_transcript(&self) -> Transcript<M, P> {
        let moves = self
            .map_with_index(|p, m| Played::player(p, m))
            .into_iter()
            .collect();
        Transcript::from_played_moves(moves)
    }
}

/// An iterator over available moves in a game with a finite move set.
#[derive(Clone)]
pub struct MoveIter<'a, M> {
    iter: Box<dyn CloneableIterator<M> + 'a>,
}

/// An iterator that can be cloned, enabling it to be used multiple times.
///
/// A blanket implementation covers all types that meet the requirements, so this trait should not
/// be implemented directly.
trait CloneableIterator<I>: DynClone + Iterator<Item = I> {}
impl<I, T: DynClone + Iterator<Item = I>> CloneableIterator<I> for T {}

dyn_clone::clone_trait_object!(<I> CloneableIterator<I>);

impl<'a, M> MoveIter<'a, M> {
    /// Construct a new move iterator.
    pub fn new(iter: impl Clone + Iterator<Item = M> + 'a) -> Self {
        MoveIter {
            iter: Box::new(iter),
        }
    }
}

impl<'a, M> Iterator for MoveIter<'a, M> {
    type Item = M;
    fn next(&mut self) -> Option<M> {
        self.iter.next()
    }
}

/// An iterator over all pure strategy profiles for a
/// [simultaneous, finite-move])(crate::game::sim::SimFin) game.
///
/// This iterator enumerates all profiles that can be produced from the moves available to each
/// player.
#[derive(Clone)]
pub struct ProfileIter<'g, M: Copy, const P: usize> {
    /// Moves that must be included in any generated profile, for each player.
    includes: PerPlayer<Vec<M>, P>,
    /// Moves that must be excluded from any generated profile, for each player.
    excludes: PerPlayer<Vec<M>, P>,
    /// The multi-product iterator used to generate each profile.
    multi_iter: MultiProduct<MoveIter<'g, M>>,
}

impl<'g, M: Move, const P: usize> ProfileIter<'g, M, P> {
    /// Construct a new profile iterator from a per-player collection of iterators over the moves
    /// available to each player.
    ///
    /// # Examples
    /// ```
    /// use tft::norm::*;
    ///
    /// let move_iters = PerPlayer::new([
    ///     MoveIter::new(vec!['A', 'B', 'C'].into_iter()),
    ///     MoveIter::new(vec!['D', 'E'].into_iter()),
    ///     MoveIter::new(vec!['F', 'G'].into_iter()),
    /// ]);
    /// let iter = ProfileIter::from_move_iters(move_iters);
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
    pub fn from_move_iters(move_iters: PerPlayer<MoveIter<'g, M>, P>) -> Self {
        ProfileIter {
            includes: PerPlayer::init_with(Vec::new()),
            excludes: PerPlayer::init_with(Vec::new()),
            multi_iter: move_iters.into_iter().multi_cartesian_product(),
        }
    }

    /// Construct a new profile iterator from a per-player collection of vectors of available moves
    /// for each player.
    ///
    /// # Examples
    /// ```
    /// use tft::norm::*;
    ///
    /// let moves = PerPlayer::new([
    ///     vec!['A', 'B', 'C', 'D'],
    ///     vec!['E', 'F', 'G'],
    /// ]);
    /// let mut iter = ProfileIter::from_move_vecs(moves).include(for2::P1, 'F');
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['A', 'F'])));
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['B', 'F'])));
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['C', 'F'])));
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['D', 'F'])));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn from_move_vecs(move_vecs: PerPlayer<Vec<M>, P>) -> ProfileIter<'static, M, P> {
        ProfileIter::from_move_iters(move_vecs.map(|v| MoveIter::new(v.into_iter())))
    }

    /// Construct a new profile iterator for a game where each player has the same set of available
    /// moves.
    ///
    /// # Examples
    ///
    /// Generating all profiles for a symmetric 2-player game:
    /// ```
    /// use tft::norm::*;
    ///
    /// let move_iter = MoveIter::new(vec!['X', 'O'].into_iter());
    /// let iter = ProfileIter::symmetric(move_iter);
    /// assert_eq!(
    ///     iter.collect::<Vec<PerPlayer<char, 2>>>(),
    ///     vec![
    ///         PerPlayer::new(['X', 'X']), PerPlayer::new(['X', 'O']),
    ///         PerPlayer::new(['O', 'X']), PerPlayer::new(['O', 'O']),
    ///     ],
    /// );
    ///
    /// let move_iter = MoveIter::new(vec!['A', 'B', 'C'].into_iter());
    /// let iter = ProfileIter::symmetric(move_iter);
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
    /// use tft::norm::*;
    ///
    /// let move_iter = MoveIter::new(vec!['X', 'O'].into_iter());
    /// let iter = ProfileIter::symmetric(move_iter);
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
    pub fn symmetric(move_iter: MoveIter<'g, M>) -> Self {
        ProfileIter::from_move_iters(PerPlayer::init_with(move_iter))
    }

    /// Constrain the iterator to generate only profiles where the given player plays a specific
    /// move.
    ///
    /// If the move is not a valid move for that player, then the resulting iterator will not
    /// generate any profiles.
    ///
    /// Multiple invocations of [`include`](ProfileIter::include) and
    /// [`exclude`](ProfileIter::exclude) can be chained together to add several constraints to
    /// the iterator.
    ///
    /// # Examples
    ///
    /// Constraining a single player's move:
    /// ```
    /// use tft::norm::*;
    ///
    /// let moves = PerPlayer::new([
    ///     vec!['A', 'B'],
    ///     vec!['C', 'D', 'E'],
    /// ]);
    ///
    /// let mut iter = ProfileIter::from_move_vecs(moves.clone()).include(for2::P0, 'B');
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['B', 'C'])));
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['B', 'D'])));
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['B', 'E'])));
    /// assert_eq!(iter.next(), None);
    ///
    /// let mut iter = ProfileIter::from_move_vecs(moves).include(for2::P1, 'D');
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['A', 'D'])));
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['B', 'D'])));
    /// assert_eq!(iter.next(), None);
    /// ```
    ///
    /// Constraining multiple players' moves by chaining invocations of this method:
    /// ```
    /// use tft::norm::*;
    ///
    /// let move_iter = MoveIter::new(vec!['A', 'B', 'C'].into_iter());
    /// let mut iter = ProfileIter::symmetric(move_iter)
    ///     .include(for3::P0, 'A')
    ///     .include(for3::P2, 'C');
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['A', 'A', 'C'])));
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['A', 'B', 'C'])));
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['A', 'C', 'C'])));
    /// assert_eq!(iter.next(), None);
    /// ```
    ///
    /// Combining with [`exclude`](ProfileIter::exclude):
    /// ```
    /// use tft::norm::*;
    ///
    /// let moves = PerPlayer::new([
    ///     vec!['A', 'B'],
    ///     vec!['C', 'D', 'E'],
    ///     vec!['F', 'G', 'H'],
    /// ]);
    ///
    /// let mut iter = ProfileIter::from_move_vecs(moves.clone())
    ///     .include(for3::P1, 'D')
    ///     .exclude(for3::P2, 'G');
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['A', 'D', 'F'])));
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['A', 'D', 'H'])));
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['B', 'D', 'F'])));
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['B', 'D', 'H'])));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn include(self, player: PlayerIndex<P>, the_move: M) -> Self {
        let mut includes = self.includes;
        includes[player].push(the_move);
        ProfileIter { includes, ..self }
    }

    /// Constrain the iterator to generate only profiles where the given player *does not* play a
    /// specific move.
    ///
    /// If the move is not a valid move for that player, then this method will have no effect.
    ///
    /// Multiple invocations of [`include`](ProfileIter::include) and
    /// [`exclude`](ProfileIter::exclude) can be chained together to add several constraints to
    /// the iterator.
    ///
    /// # Examples
    ///
    /// Applying a single exlcusion constraint:
    /// ```
    /// use tft::norm::*;
    ///
    /// let moves = PerPlayer::new([
    ///     vec!['A', 'B'],
    ///     vec!['C', 'D', 'E'],
    /// ]);
    ///
    /// let mut iter = ProfileIter::from_move_vecs(moves).exclude(for2::P1, 'C');
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['A', 'D'])));
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['A', 'E'])));
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['B', 'D'])));
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['B', 'E'])));
    /// assert_eq!(iter.next(), None);
    /// ```
    ///
    /// Applying multiple exclusion constraints by chaining invocations of this method:
    /// ```
    /// use tft::norm::*;
    ///
    /// let moves = PerPlayer::new([
    ///     vec!['A', 'B', 'C'],
    ///     vec!['D', 'E', 'F', 'G'],
    /// ]);
    ///
    /// let mut iter = ProfileIter::from_move_vecs(moves)
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
    /// Combining with [`include`](ProfileIter::include):
    /// ```
    /// use tft::norm::*;
    ///
    /// let move_iter = MoveIter::new(vec!['A', 'B', 'C'].into_iter());
    /// let mut iter = ProfileIter::symmetric(move_iter)
    ///     .exclude(for3::P0, 'A')
    ///     .exclude(for3::P1, 'B')
    ///     .include(for3::P2, 'C');
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['B', 'A', 'C'])));
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['B', 'C', 'C'])));
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['C', 'A', 'C'])));
    /// assert_eq!(iter.next(), Some(PerPlayer::new(['C', 'C', 'C'])));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn exclude(self, player: PlayerIndex<P>, the_move: M) -> Self {
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
    /// use tft::norm::*;
    ///
    /// let move_iter = MoveIter::new(vec![1, 2, 3, 4, 5].into_iter());
    /// let mut iter = ProfileIter::symmetric(move_iter)
    ///     .adjacent(for5::P3, PerPlayer::new([5, 4, 3, 2, 1]));
    /// assert_eq!(iter.next(), Some(PerPlayer::new([5, 4, 3, 1, 1])));
    /// assert_eq!(iter.next(), Some(PerPlayer::new([5, 4, 3, 3, 1])));
    /// assert_eq!(iter.next(), Some(PerPlayer::new([5, 4, 3, 4, 1])));
    /// assert_eq!(iter.next(), Some(PerPlayer::new([5, 4, 3, 5, 1])));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn adjacent(self, player: PlayerIndex<P>, profile: Profile<M, P>) -> Self {
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

impl<'g, M: Move, const P: usize> Iterator for ProfileIter<'g, M, P> {
    type Item = Profile<M, P>;

    fn next(&mut self) -> Option<Profile<M, P>> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{for3, PerPlayer};
    use test_log::test;

    #[test]
    fn adjacent_profiles_for3_correct() {
        let iter = ProfileIter::from_move_vecs(PerPlayer::new([
            vec!['A', 'B'],
            vec!['C', 'D', 'E'],
            vec!['F', 'G', 'H', 'I'],
        ]));

        let profile1 = PerPlayer::new(['A', 'C', 'F']);
        let profile2 = PerPlayer::new(['B', 'D', 'I']);
        let profile3 = PerPlayer::new(['A', 'E', 'G']);

        assert_eq!(
            iter.clone()
                .adjacent(for3::P0, profile1)
                .collect::<Vec<Profile<char, 3>>>(),
            vec![PerPlayer::new(['B', 'C', 'F'])],
        );
        assert_eq!(
            iter.clone()
                .adjacent(for3::P0, profile2)
                .collect::<Vec<Profile<char, 3>>>(),
            vec![PerPlayer::new(['A', 'D', 'I'])],
        );
        assert_eq!(
            iter.clone()
                .adjacent(for3::P0, profile3)
                .collect::<Vec<Profile<char, 3>>>(),
            vec![PerPlayer::new(['B', 'E', 'G'])],
        );
        assert_eq!(
            iter.clone()
                .adjacent(for3::P1, profile1)
                .collect::<Vec<Profile<char, 3>>>(),
            vec![
                PerPlayer::new(['A', 'D', 'F']),
                PerPlayer::new(['A', 'E', 'F'])
            ],
        );
        assert_eq!(
            iter.clone()
                .adjacent(for3::P1, profile2)
                .collect::<Vec<Profile<char, 3>>>(),
            vec![
                PerPlayer::new(['B', 'C', 'I']),
                PerPlayer::new(['B', 'E', 'I'])
            ],
        );
        assert_eq!(
            iter.clone()
                .adjacent(for3::P1, profile3)
                .collect::<Vec<Profile<char, 3>>>(),
            vec![
                PerPlayer::new(['A', 'C', 'G']),
                PerPlayer::new(['A', 'D', 'G'])
            ],
        );
        assert_eq!(
            iter.clone()
                .adjacent(for3::P2, profile1)
                .collect::<Vec<Profile<char, 3>>>(),
            vec![
                PerPlayer::new(['A', 'C', 'G']),
                PerPlayer::new(['A', 'C', 'H']),
                PerPlayer::new(['A', 'C', 'I']),
            ],
        );
        assert_eq!(
            iter.clone()
                .adjacent(for3::P2, profile2)
                .collect::<Vec<Profile<char, 3>>>(),
            vec![
                PerPlayer::new(['B', 'D', 'F']),
                PerPlayer::new(['B', 'D', 'G']),
                PerPlayer::new(['B', 'D', 'H']),
            ],
        );
        assert_eq!(
            iter.adjacent(for3::P2, profile3)
                .collect::<Vec<Profile<char, 3>>>(),
            vec![
                PerPlayer::new(['A', 'E', 'F']),
                PerPlayer::new(['A', 'E', 'H']),
                PerPlayer::new(['A', 'E', 'I']),
            ],
        );
    }
}
