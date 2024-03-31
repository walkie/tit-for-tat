use crate::{Move, PerPlayer, PlayerIndex, PossibleMoves, Profile};
use itertools::{Itertools, MultiProduct};

/// An iterator over all pure strategy profiles for a [normal-form game](crate::Normal).
///
/// This iterator enumerates all profiles that can be produced from the moves available to each
/// player.
#[derive(Clone)]
pub struct PossibleProfiles<'g, M: Copy, const P: usize> {
    /// Moves that must be included in any generated profile, for each player.
    includes: PerPlayer<Vec<M>, P>,
    /// Moves that must be excluded from any generated profile, for each player.
    excludes: PerPlayer<Vec<M>, P>,
    /// The multi-product iterator used to generate each profile.
    multi_iter: MultiProduct<PossibleMoves<'g, M>>,
}

impl<'g, M: Move, const P: usize> PossibleProfiles<'g, M, P> {
    /// Construct a new profile iterator from a per-player collection of iterators over the moves
    /// available to each player.
    ///
    /// # Examples
    /// ```
    /// use t4t::*;
    ///
    /// let moves = PerPlayer::new([
    ///     PossibleMoves::from_vec(vec!['A', 'B', 'C']),
    ///     PossibleMoves::from_vec(vec!['D', 'E']),
    ///     PossibleMoves::from_vec(vec!['F', 'G']),
    /// ]);
    /// let profiles = PossibleProfiles::from_move_iters(moves);
    /// assert_eq!(
    ///     profiles.collect::<Vec<Profile<char, 3>>>(),
    ///     vec![
    ///         Profile::new(['A', 'D', 'F']), Profile::new(['A', 'D', 'G']),
    ///         Profile::new(['A', 'E', 'F']), Profile::new(['A', 'E', 'G']),
    ///         Profile::new(['B', 'D', 'F']), Profile::new(['B', 'D', 'G']),
    ///         Profile::new(['B', 'E', 'F']), Profile::new(['B', 'E', 'G']),
    ///         Profile::new(['C', 'D', 'F']), Profile::new(['C', 'D', 'G']),
    ///         Profile::new(['C', 'E', 'F']), Profile::new(['C', 'E', 'G']),
    ///     ],
    /// );
    /// ```
    pub fn from_move_iters(move_iters: PerPlayer<PossibleMoves<'g, M>, P>) -> Self {
        PossibleProfiles {
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
    /// use t4t::*;
    ///
    /// let moves = PerPlayer::new([
    ///     vec!['A', 'B', 'C', 'D'],
    ///     vec!['E', 'F', 'G'],
    /// ]);
    /// let mut profiles = PossibleProfiles::from_move_vecs(moves).include(for2::P1, 'F');
    /// assert_eq!(profiles.next(), Some(Profile::new(['A', 'F'])));
    /// assert_eq!(profiles.next(), Some(Profile::new(['B', 'F'])));
    /// assert_eq!(profiles.next(), Some(Profile::new(['C', 'F'])));
    /// assert_eq!(profiles.next(), Some(Profile::new(['D', 'F'])));
    /// assert_eq!(profiles.next(), None);
    /// ```
    pub fn from_move_vecs(move_vecs: PerPlayer<Vec<M>, P>) -> PossibleProfiles<'static, M, P> {
        PossibleProfiles::from_move_iters(move_vecs.map(|v| PossibleMoves::from_vec(v)))
    }

    /// Construct a new profile iterator for a game where each player has the same set of available
    /// moves.
    ///
    /// # Examples
    ///
    /// Generating all profiles for a symmetric 2-player game:
    /// ```
    /// use t4t::*;
    ///
    /// let moves = PossibleMoves::from_vec(vec!['X', 'O']);
    /// let profiles = PossibleProfiles::symmetric(moves);
    /// assert_eq!(
    ///     profiles.collect::<Vec<Profile<char, 2>>>(),
    ///     vec![
    ///         Profile::new(['X', 'X']), Profile::new(['X', 'O']),
    ///         Profile::new(['O', 'X']), Profile::new(['O', 'O']),
    ///     ],
    /// );
    ///
    /// let moves = PossibleMoves::from_vec(vec!['A', 'B', 'C']);
    /// let profiles = PossibleProfiles::symmetric(moves);
    /// assert_eq!(
    ///     profiles.collect::<Vec<Profile<char, 2>>>(),
    ///     vec![
    ///         Profile::new(['A', 'A']), Profile::new(['A', 'B']), Profile::new(['A', 'C']),
    ///         Profile::new(['B', 'A']), Profile::new(['B', 'B']), Profile::new(['B', 'C']),
    ///         Profile::new(['C', 'A']), Profile::new(['C', 'B']), Profile::new(['C', 'C']),
    ///     ],
    /// );
    /// ```
    ///
    /// Generating all profiles for a symmetric 3-player game:
    /// ```
    /// use t4t::*;
    ///
    /// let moves = PossibleMoves::from_vec(vec!['X', 'O']);
    /// let profiles = PossibleProfiles::symmetric(moves);
    /// assert_eq!(
    ///     profiles.collect::<Vec<Profile<char, 3>>>(),
    ///     vec![
    ///         Profile::new(['X', 'X', 'X']), Profile::new(['X', 'X', 'O']),
    ///         Profile::new(['X', 'O', 'X']), Profile::new(['X', 'O', 'O']),
    ///         Profile::new(['O', 'X', 'X']), Profile::new(['O', 'X', 'O']),
    ///         Profile::new(['O', 'O', 'X']), Profile::new(['O', 'O', 'O']),
    ///     ],
    /// );
    /// ```
    pub fn symmetric(move_iter: PossibleMoves<'g, M>) -> Self {
        PossibleProfiles::from_move_iters(PerPlayer::init_with(move_iter))
    }

    /// Constrain the iterator to generate only profiles where the given player plays a specific
    /// move.
    ///
    /// If the move is not a valid move for that player, then the resulting iterator will not
    /// generate any profiles.
    ///
    /// Multiple invocations of [`include`](PossibleProfiles::include) and
    /// [`exclude`](PossibleProfiles::exclude) can be chained together to add several constraints to
    /// the iterator.
    ///
    /// # Examples
    ///
    /// Constraining a single player's move:
    /// ```
    /// use t4t::*;
    ///
    /// let moves = PerPlayer::new([
    ///     vec!['A', 'B'],
    ///     vec!['C', 'D', 'E'],
    /// ]);
    ///
    /// let mut profiles = PossibleProfiles::from_move_vecs(moves.clone()).include(for2::P0, 'B');
    /// assert_eq!(profiles.next(), Some(Profile::new(['B', 'C'])));
    /// assert_eq!(profiles.next(), Some(Profile::new(['B', 'D'])));
    /// assert_eq!(profiles.next(), Some(Profile::new(['B', 'E'])));
    /// assert_eq!(profiles.next(), None);
    ///
    /// let mut profiles = PossibleProfiles::from_move_vecs(moves).include(for2::P1, 'D');
    /// assert_eq!(profiles.next(), Some(Profile::new(['A', 'D'])));
    /// assert_eq!(profiles.next(), Some(Profile::new(['B', 'D'])));
    /// assert_eq!(profiles.next(), None);
    /// ```
    ///
    /// Constraining multiple players' moves by chaining invocations of this method:
    /// ```
    /// use t4t::*;
    ///
    /// let moves = PossibleMoves::from_vec(vec!['A', 'B', 'C']);
    /// let mut profiles = PossibleProfiles::symmetric(moves)
    ///     .include(for3::P0, 'A')
    ///     .include(for3::P2, 'C');
    /// assert_eq!(profiles.next(), Some(Profile::new(['A', 'A', 'C'])));
    /// assert_eq!(profiles.next(), Some(Profile::new(['A', 'B', 'C'])));
    /// assert_eq!(profiles.next(), Some(Profile::new(['A', 'C', 'C'])));
    /// assert_eq!(profiles.next(), None);
    /// ```
    ///
    /// Combining with [`exclude`](PossibleProfiles::exclude):
    /// ```
    /// use t4t::*;
    ///
    /// let moves = PerPlayer::new([
    ///     vec!['A', 'B'],
    ///     vec!['C', 'D', 'E'],
    ///     vec!['F', 'G', 'H'],
    /// ]);
    ///
    /// let mut profiles = PossibleProfiles::from_move_vecs(moves.clone())
    ///     .include(for3::P1, 'D')
    ///     .exclude(for3::P2, 'G');
    /// assert_eq!(profiles.next(), Some(Profile::new(['A', 'D', 'F'])));
    /// assert_eq!(profiles.next(), Some(Profile::new(['A', 'D', 'H'])));
    /// assert_eq!(profiles.next(), Some(Profile::new(['B', 'D', 'F'])));
    /// assert_eq!(profiles.next(), Some(Profile::new(['B', 'D', 'H'])));
    /// assert_eq!(profiles.next(), None);
    /// ```
    pub fn include(self, player: PlayerIndex<P>, the_move: M) -> Self {
        let mut includes = self.includes;
        includes[player].push(the_move);
        PossibleProfiles { includes, ..self }
    }

    /// Constrain the iterator to generate only profiles where the given player *does not* play a
    /// specific move.
    ///
    /// If the move is not a valid move for that player, then this method will have no effect.
    ///
    /// Multiple invocations of [`include`](PossibleProfiles::include) and
    /// [`exclude`](PossibleProfiles::exclude) can be chained together to add several constraints to
    /// the iterator.
    ///
    /// # Examples
    ///
    /// Applying a single exclusion constraint:
    /// ```
    /// use t4t::*;
    ///
    /// let moves = PerPlayer::new([
    ///     vec!['A', 'B'],
    ///     vec!['C', 'D', 'E'],
    /// ]);
    ///
    /// let mut profiles = PossibleProfiles::from_move_vecs(moves).exclude(for2::P1, 'C');
    /// assert_eq!(profiles.next(), Some(Profile::new(['A', 'D'])));
    /// assert_eq!(profiles.next(), Some(Profile::new(['A', 'E'])));
    /// assert_eq!(profiles.next(), Some(Profile::new(['B', 'D'])));
    /// assert_eq!(profiles.next(), Some(Profile::new(['B', 'E'])));
    /// assert_eq!(profiles.next(), None);
    /// ```
    ///
    /// Applying multiple exclusion constraints by chaining invocations of this method:
    /// ```
    /// use t4t::*;
    ///
    /// let moves = PerPlayer::new([
    ///     vec!['A', 'B', 'C'],
    ///     vec!['D', 'E', 'F', 'G'],
    /// ]);
    ///
    /// let mut profiles = PossibleProfiles::from_move_vecs(moves)
    ///     .exclude(for2::P0, 'A')
    ///     .exclude(for2::P1, 'E')
    ///     .exclude(for2::P1, 'G');
    /// assert_eq!(profiles.next(), Some(Profile::new(['B', 'D'])));
    /// assert_eq!(profiles.next(), Some(Profile::new(['B', 'F'])));
    /// assert_eq!(profiles.next(), Some(Profile::new(['C', 'D'])));
    /// assert_eq!(profiles.next(), Some(Profile::new(['C', 'F'])));
    /// assert_eq!(profiles.next(), None);
    /// ```
    ///
    /// Combining with [`include`](PossibleProfiles::include):
    /// ```
    /// use t4t::*;
    ///
    /// let moves = PossibleMoves::from_vec(vec!['A', 'B', 'C']);
    /// let mut profiles = PossibleProfiles::symmetric(moves)
    ///     .exclude(for3::P0, 'A')
    ///     .exclude(for3::P1, 'B')
    ///     .include(for3::P2, 'C');
    /// assert_eq!(profiles.next(), Some(Profile::new(['B', 'A', 'C'])));
    /// assert_eq!(profiles.next(), Some(Profile::new(['B', 'C', 'C'])));
    /// assert_eq!(profiles.next(), Some(Profile::new(['C', 'A', 'C'])));
    /// assert_eq!(profiles.next(), Some(Profile::new(['C', 'C', 'C'])));
    /// assert_eq!(profiles.next(), None);
    /// ```
    pub fn exclude(self, player: PlayerIndex<P>, the_move: M) -> Self {
        let mut excludes = self.excludes;
        excludes[player].push(the_move);
        PossibleProfiles { excludes, ..self }
    }

    /// Constrain the iterator to generate only profiles that are "adjacent" to the given profile
    /// for a given player.
    ///
    /// An adjacent profile is one where the given player plays a different move, but all other
    /// players play the move specified in the profile.
    ///
    /// # Examples
    /// ```
    /// use t4t::*;
    ///
    /// let moves = PossibleMoves::from_vec(vec![1, 2, 3, 4, 5]);
    /// let mut profiles = PossibleProfiles::symmetric(moves)
    ///     .adjacent(for5::P3, Profile::new([5, 4, 3, 2, 1]));
    /// assert_eq!(profiles.next(), Some(Profile::new([5, 4, 3, 1, 1])));
    /// assert_eq!(profiles.next(), Some(Profile::new([5, 4, 3, 3, 1])));
    /// assert_eq!(profiles.next(), Some(Profile::new([5, 4, 3, 4, 1])));
    /// assert_eq!(profiles.next(), Some(Profile::new([5, 4, 3, 5, 1])));
    /// assert_eq!(profiles.next(), None);
    /// ```
    pub fn adjacent(self, player: PlayerIndex<P>, profile: Profile<M, P>) -> Self {
        let mut iter = self;
        for i in PlayerIndex::all() {
            if i == player {
                iter = iter.exclude(i, profile[i]);
            } else {
                iter = iter.include(i, profile[i]);
            }
        }
        iter
    }
}

impl<'g, M: Move, const P: usize> Iterator for PossibleProfiles<'g, M, P> {
    type Item = Profile<M, P>;

    fn next(&mut self) -> Option<Profile<M, P>> {
        for moves in self.multi_iter.by_ref() {
            let profile = Profile::new(moves.try_into().unwrap());
            let mut good = true;
            for player in PlayerIndex::all() {
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
        let profiles = PossibleProfiles::from_move_vecs(PerPlayer::new([
            vec!['A', 'B'],
            vec!['C', 'D', 'E'],
            vec!['F', 'G', 'H', 'I'],
        ]));

        let profile1 = Profile::new(['A', 'C', 'F']);
        let profile2 = Profile::new(['B', 'D', 'I']);
        let profile3 = Profile::new(['A', 'E', 'G']);

        assert_eq!(
            profiles
                .clone()
                .adjacent(for3::P0, profile1)
                .collect::<Vec<Profile<char, 3>>>(),
            vec![Profile::new(['B', 'C', 'F'])],
        );
        assert_eq!(
            profiles
                .clone()
                .adjacent(for3::P0, profile2)
                .collect::<Vec<Profile<char, 3>>>(),
            vec![Profile::new(['A', 'D', 'I'])],
        );
        assert_eq!(
            profiles
                .clone()
                .adjacent(for3::P0, profile3)
                .collect::<Vec<Profile<char, 3>>>(),
            vec![Profile::new(['B', 'E', 'G'])],
        );
        assert_eq!(
            profiles
                .clone()
                .adjacent(for3::P1, profile1)
                .collect::<Vec<Profile<char, 3>>>(),
            vec![Profile::new(['A', 'D', 'F']), Profile::new(['A', 'E', 'F'])],
        );
        assert_eq!(
            profiles
                .clone()
                .adjacent(for3::P1, profile2)
                .collect::<Vec<Profile<char, 3>>>(),
            vec![Profile::new(['B', 'C', 'I']), Profile::new(['B', 'E', 'I'])],
        );
        assert_eq!(
            profiles
                .clone()
                .adjacent(for3::P1, profile3)
                .collect::<Vec<Profile<char, 3>>>(),
            vec![Profile::new(['A', 'C', 'G']), Profile::new(['A', 'D', 'G'])],
        );
        assert_eq!(
            profiles
                .clone()
                .adjacent(for3::P2, profile1)
                .collect::<Vec<Profile<char, 3>>>(),
            vec![
                Profile::new(['A', 'C', 'G']),
                Profile::new(['A', 'C', 'H']),
                Profile::new(['A', 'C', 'I']),
            ],
        );
        assert_eq!(
            profiles
                .clone()
                .adjacent(for3::P2, profile2)
                .collect::<Vec<Profile<char, 3>>>(),
            vec![
                Profile::new(['B', 'D', 'F']),
                Profile::new(['B', 'D', 'G']),
                Profile::new(['B', 'D', 'H']),
            ],
        );
        assert_eq!(
            profiles
                .adjacent(for3::P2, profile3)
                .collect::<Vec<Profile<char, 3>>>(),
            vec![
                Profile::new(['A', 'E', 'F']),
                Profile::new(['A', 'E', 'H']),
                Profile::new(['A', 'E', 'I']),
            ],
        );
    }
}
