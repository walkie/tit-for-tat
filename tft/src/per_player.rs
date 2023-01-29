//! This module defines the [`PerPlayer`] collection type that stores one element for each player
//! in a game.

use derive_more::{AsMut, AsRef};
use std::iter::IntoIterator;
use std::ops::{Index, IndexMut};

/// A collection that stores one element corresponding to each player in a game.
///
/// The type is parameterized by the type of elements `T` and the number of players in the game
/// `N`. For example, the type `PerPlayer<f64, 3>` contains exactly three `f64` values, one for
/// each player in a three-player game.
///
/// The ["const generic"](https://blog.rust-lang.org/2021/02/26/const-generics-mvp-beta.html)
/// argument `N` is used to statically ensure that a [`PerPlayer`] collection contains the correct
/// number of elements for a given game, and to provide statically checked indexing into
/// `PerPlayer` collections.
///
/// # Dynamically checked indexing operations
///
/// The [`for_player`](PerPlayer::for_player) and [`for_player_mut`](PerPlayer::for_player_mut)
/// methods allow indexing into a `PerPlayer` collection with plain `usize` indexes. They return
/// references wrapped in an [`Option`] type, which may be `None` if the index is too large for the
/// number of players in the game.
///
/// ```
/// use tft::PerPlayer;
///
/// let mut pp = PerPlayer::new(["klaatu", "barada", "nikto"]);
/// assert_eq!(pp.for_player(0), Some(&"klaatu"));
/// assert_eq!(pp.for_player(1), Some(&"barada"));
/// assert_eq!(pp.for_player(2), Some(&"nikto"));
/// assert_eq!(pp.for_player(3), None);
///
/// *pp.for_player_mut(0).unwrap() = "gort";
/// assert_eq!(pp.for_player(0), Some(&"gort"));
/// ```
///
/// # Statically checked indexing operations
///
/// The [`Index`] and [`IndexMut`] traits are implemented for `PerPlayer` collections with indexes
/// of type [`PlayerIndex`]. An index of type `PlayerIndex<N>` is guaranteed to be in-range for a
/// collection of type `PerPlayer<T, N>`, that is, indexing operations into a `PerPlayer`
/// collection are guaranteed not to fail due to an index out-of-bounds error.
///
/// Indexes can be constructed dynamically using the [`PlayerIndex::new`] constructor. While the
/// *indexing operation* cannot fail, *constructing an index* may fail if the index is out of
/// bounds, in which case the constructor will return `None`.
///
/// ```
/// use tft::PlayerIndex;
///
/// assert!(PlayerIndex::<3>::new(0).is_some());
/// assert!(PlayerIndex::<3>::new(1).is_some());
/// assert!(PlayerIndex::<3>::new(2).is_some());
/// assert!(PlayerIndex::<3>::new(3).is_none());
/// ```
///
/// When constructing indexes, often the value of `N` can be inferred from the type of the
/// `PerPlayer` collection it is used to index into.
///
/// ```
/// use tft::{PerPlayer, PlayerIndex};
///
/// let mut pp = PerPlayer::new(["klaatu", "barada", "nikto"]);
/// let p0 = PlayerIndex::new(0).unwrap();
/// let p1 = PlayerIndex::new(1).unwrap();
/// let p2 = PlayerIndex::new(2).unwrap();
/// assert_eq!(pp[p0], "klaatu");
/// assert_eq!(pp[p1], "barada");
/// assert_eq!(pp[p2], "nikto");
///
/// pp[p0] = "gort";
/// assert_eq!(pp[p0], "gort");
/// ```
///
/// Additionally, this module contains several submodules that predefine named indexes for all
/// players up to a player count of 16. For example, the indexes for three player games are
/// included in the [`for3`] submodule.
///
/// ```
/// use tft::{for3, PerPlayer};
///
/// let mut pp = PerPlayer::new(["klaatu", "barada", "nikto"]);
/// assert_eq!(pp[for3::P0], "klaatu");
/// assert_eq!(pp[for3::P1], "barada");
/// assert_eq!(pp[for3::P2], "nikto");
///
/// pp[for3::P0] = "gort";
/// assert_eq!(pp[for3::P0], "gort");
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, AsMut, AsRef)]
pub struct PerPlayer<T, const N: usize> {
    data: [T; N],
}

impl<T, const N: usize> PerPlayer<T, N> {
    /// Create a new per-player collection from the given array.
    pub fn new(data: [T; N]) -> Self {
        PerPlayer { data }
    }

    /// Create a new per-player collection by calling the given function with each player index,
    /// collecting the results.
    ///
    /// # Examples
    /// ```
    /// use tft::{for5, PerPlayer, PlayerIndex};
    ///
    /// let squared = |index: PlayerIndex<5>| {
    ///     let val: usize = index.into();
    ///     val * val
    /// };
    /// let squares = PerPlayer::generate(squared);
    /// assert_eq!(squares[for5::P0], 0);
    /// assert_eq!(squares[for5::P1], 1);
    /// assert_eq!(squares[for5::P2], 4);
    /// assert_eq!(squares[for5::P3], 9);
    /// assert_eq!(squares[for5::P4], 16);
    /// ```
    pub fn generate<F: FnMut(PlayerIndex<N>) -> T>(gen_elem: F) -> Self {
        let indexes: [PlayerIndex<N>; N] = PlayerIndex::all_indexes()
            .collect::<Vec<PlayerIndex<N>>>()
            .try_into()
            .unwrap();
        PerPlayer::new(indexes.map(gen_elem))
    }

    /// Get the number of players in the game, which corresponds to the number of elements in this
    /// collection.
    pub fn num_players(&self) -> usize {
        N
    }

    /// Get a reference to the element corresponding to the `i`th player in the game. Returns
    /// `None` if the index is out of range.
    ///
    /// # Examples
    /// ```
    /// use tft::PerPlayer;
    ///
    /// let mut pp = PerPlayer::new(["frodo", "sam", "merry", "pippin"]);
    /// assert_eq!(pp.for_player(0), Some(&"frodo"));
    /// assert_eq!(pp.for_player(1), Some(&"sam"));
    /// assert_eq!(pp.for_player(2), Some(&"merry"));
    /// assert_eq!(pp.for_player(3), Some(&"pippin"));
    /// assert_eq!(pp.for_player(4), None);
    /// ```
    pub fn for_player(&self, i: usize) -> Option<&T> {
        if i < N {
            Some(&self.data[i])
        } else {
            log::warn!("PerPlayer<{}>::for_player({}): index out of range", N, i);
            None
        }
    }

    /// Get a mutable reference to the element corresponding to the `i`th player in the game.
    /// Returns `None` if the index is out of range.
    ///
    /// # Examples
    /// ```
    /// use tft::PerPlayer;
    ///
    /// let mut pp = PerPlayer::new(["frodo", "sam", "merry", "pippin"]);
    /// *pp.for_player_mut(1).unwrap() = "samwise";
    /// *pp.for_player_mut(2).unwrap() = "meriadoc";
    /// *pp.for_player_mut(3).unwrap() = "peregrin";
    /// assert_eq!(pp.for_player(0), Some(&"frodo"));
    /// assert_eq!(pp.for_player(1), Some(&"samwise"));
    /// assert_eq!(pp.for_player(2), Some(&"meriadoc"));
    /// assert_eq!(pp.for_player(3), Some(&"peregrin"));
    /// assert_eq!(pp.for_player(4), None);
    /// ```
    pub fn for_player_mut(&mut self, i: usize) -> Option<&mut T> {
        if i < N {
            Some(&mut self.data[i])
        } else {
            log::warn!(
                "PerPlayer<{}>::for_player_mut({}): index out of range",
                N,
                i
            );
            None
        }
    }
}

impl<T: Clone, const N: usize> PerPlayer<T, N> {
    /// Create a new per-player collection where each element is initialized with the given
    /// cloneable value.
    pub fn init_with(value: T) -> Self {
        PerPlayer::generate(|_| value.clone())
    }

    /// Map a function over all of the elements in a per-player collection, producing a new
    /// per-player collection.
    ///
    /// # Examples
    /// ```
    /// use tft::PerPlayer;
    ///
    /// let mut pp = PerPlayer::new(["frodo", "sam", "merry", "pippin"]);
    ///
    /// let mut lengths = pp.map(|s| s.len());
    /// assert_eq!(lengths, PerPlayer::new([5, 3, 5, 6]));
    ///
    /// let mut firsts = pp.map(|s| s.chars().next().unwrap());
    /// assert_eq!(firsts, PerPlayer::new(['f', 's', 'm', 'p']));
    /// ```
    pub fn map<U, F: FnMut(T) -> U>(&self, f: F) -> PerPlayer<U, N> {
        PerPlayer::new(self.data.clone().map(f))
    }

    /// Map a function over all of the elements in a per-player collection, producing a new
    /// per-player collection.
    ///
    /// This variant of map provides each element's index along with the element to the mapped
    /// function.
    ///
    /// # Examples
    /// ```
    /// use tft::PerPlayer;
    ///
    /// let mut pp = PerPlayer::new(["frodo", "sam", "merry", "pippin"]);
    ///
    /// let mut pairs = pp.map_with_index(|i, s| (i.as_usize(), s.len()));
    /// assert_eq!(pairs, PerPlayer::new([(0, 5), (1, 3), (2, 5), (3, 6)]));
    ///
    /// let mut nths = pp.map_with_index(|i, s| s.chars().nth(i.as_usize()).unwrap());
    /// assert_eq!(nths, PerPlayer::new(['f', 'a', 'r', 'p']));
    /// ```
    pub fn map_with_index<U, F: Fn(PlayerIndex<N>, T) -> U>(&self, f: F) -> PerPlayer<U, N> {
        let mut indexes = PlayerIndex::all_indexes();
        PerPlayer::new(self.data.clone().map(move |elem| {
            let index = indexes.next().unwrap();
            f(index, elem)
        }))
    }
}

impl<T: core::fmt::Debug, const N: usize> PerPlayer<Option<T>, N> {
    /// Converts a per-player collection of `Option<T>` values into a per-player collection of `T`
    /// values if every element in the initial collection is `Some`; otherwise returns `None`.
    ///
    /// # Examples
    /// ```
    /// use tft::PerPlayer;
    ///
    /// assert_eq!(
    ///     PerPlayer::new([Some(3), Some(4), Some(5)]).all_some(),
    ///     Some(PerPlayer::new([3, 4, 5])),
    /// );
    /// assert_eq!(
    ///     PerPlayer::new([Some(3), None, Some(5)]).all_some(),
    ///     None,
    /// );
    /// ```
    pub fn all_some(self) -> Option<PerPlayer<T, N>> {
        self.data
            .into_iter()
            .collect::<Option<Vec<T>>>()
            .map(|vec| PerPlayer::new(vec.try_into().unwrap()))
    }
}

impl<T: Default, const N: usize> Default for PerPlayer<T, N> {
    fn default() -> Self {
        PerPlayer::generate(|_| T::default())
    }
}

impl<T, const N: usize> IntoIterator for PerPlayer<T, N> {
    type Item = <[T; N] as IntoIterator>::Item;
    type IntoIter = <[T; N] as IntoIterator>::IntoIter;
    fn into_iter(self) -> <[T; N] as IntoIterator>::IntoIter {
        self.data.into_iter()
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a PerPlayer<T, N> {
    type Item = <&'a [T; N] as IntoIterator>::Item;
    type IntoIter = <&'a [T; N] as IntoIterator>::IntoIter;
    fn into_iter(self) -> <&'a [T; N] as IntoIterator>::IntoIter {
        self.data.iter()
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a mut PerPlayer<T, N> {
    type Item = <&'a mut [T; N] as IntoIterator>::Item;
    type IntoIter = <&'a mut [T; N] as IntoIterator>::IntoIter;
    fn into_iter(self) -> <&'a mut [T; N] as IntoIterator>::IntoIter {
        self.data.iter_mut()
    }
}

/// An index into a [`PerPlayer`] collection that is guaranteed to be in-range for a game with `N`
/// players.
///
/// This type is used in the implementations of the [`Index`] and [`IndexMut`] traits and ensures
/// that their operations will not fail.
///
/// Note that player's are indexed from zero for consistency with the rest of Rust. That is, the
/// first player in a game has index `0`, the second player has index `1`, and so on. This isn't
/// ideal since most of the literature on game theory uses one-based terminology. However, I judged
/// internal consistency to be more important than external consistency, in this case. Juggling
/// multiple different indexing styles within the code itself would be really confusing!
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct PlayerIndex<const N: usize>(usize);

impl<const N: usize> PlayerIndex<N> {
    /// Construct a new index into a [`PerPlayer`] collection. Returns `None` if the provided index
    /// value is out-of-range for the number of players in the game.
    ///
    /// Predefined indexes for games of up to 16 players are defined in the `forN` modules.
    ///
    /// # Examples
    /// ```
    /// use tft::{for2, for8, PlayerIndex};
    ///
    /// let p0_opt = PlayerIndex::<2>::new(0);
    /// let p1_opt = PlayerIndex::<2>::new(1);
    /// let p2_opt = PlayerIndex::<2>::new(2);
    ///
    /// assert!(p0_opt.is_some());
    /// assert!(p1_opt.is_some());
    /// assert!(p2_opt.is_none());
    ///
    /// assert_eq!(p0_opt.unwrap(), for2::P0);
    /// assert_eq!(p1_opt.unwrap(), for2::P1);
    ///
    /// assert_eq!(PlayerIndex::<8>::new(3).unwrap(), for8::P3);
    /// assert_eq!(PlayerIndex::<8>::new(5).unwrap(), for8::P5);
    /// ```
    pub fn new(index: usize) -> Option<Self> {
        if index < N {
            Some(PlayerIndex(index))
        } else {
            log::warn!("PlayerIndex<{}>::new({}): index out of range", N, index);
            None
        }
    }

    /// Get the player index as a plain `usize` value.
    ///
    /// # Examples
    /// ```
    /// use tft::{for3, for6};
    ///
    /// assert_eq!(for3::P0.as_usize(), 0);
    /// assert_eq!(for3::P2.as_usize(), 2);
    /// assert_eq!(for6::P2.as_usize(), 2);
    /// assert_eq!(for6::P5.as_usize(), 5);
    /// ```
    pub fn as_usize(&self) -> usize {
        self.0
    }

    /// Get the number of players in the game, which corresponds to the numbers of unique indexes
    /// in this type.
    ///
    /// # Examples
    /// ```
    /// use tft::{for5, for12};
    ///
    /// assert_eq!(for5::P3.num_players(), 5);
    /// assert_eq!(for12::P7.num_players(), 12);
    /// ```
    pub fn num_players(&self) -> usize {
        N
    }

    /// Get an iterator that iterates over all player indexes of a given type.
    ///
    /// # Examples
    /// ```
    /// use tft::{for3, for5, PlayerIndex};
    ///
    /// assert_eq!(
    ///     PlayerIndex::all_indexes().collect::<Vec<PlayerIndex<3>>>(),
    ///     vec![for3::P0, for3::P1, for3::P2]
    /// );
    /// assert_eq!(
    ///     PlayerIndex::all_indexes().collect::<Vec<PlayerIndex<5>>>(),
    ///     vec![for5::P0, for5::P1, for5::P2, for5::P3, for5::P4]
    /// );
    pub fn all_indexes() -> PlayerIndexes<N> {
        PlayerIndexes::new()
    }
}

impl<const N: usize> From<PlayerIndex<N>> for usize {
    fn from(index: PlayerIndex<N>) -> usize {
        index.as_usize()
    }
}

impl<T, const N: usize> Index<PlayerIndex<N>> for PerPlayer<T, N> {
    type Output = T;
    /// Index into a `PerPlayer` collection. This operation is statically guaranteed not to fail.
    ///
    /// # Examples
    /// ```
    /// use tft::{for4, PerPlayer};
    ///
    /// let mut pp = PerPlayer::new(["frodo", "sam", "merry", "pippin"]);
    /// assert_eq!(pp[for4::P0], "frodo");
    /// assert_eq!(pp[for4::P1], "sam");
    /// assert_eq!(pp[for4::P2], "merry");
    /// assert_eq!(pp[for4::P3], "pippin");
    /// ```
    fn index(&self, idx: PlayerIndex<N>) -> &T {
        unsafe { self.data.get_unchecked(idx.0) }
    }
}

impl<T, const N: usize> IndexMut<PlayerIndex<N>> for PerPlayer<T, N> {
    /// Index into a `PerPlayer` collection in a mutable context. This operation is statically
    /// guaranteed not to fail.
    ///
    /// # Examples
    /// ```
    /// use tft::{for4, PerPlayer};
    ///
    /// let mut pp = PerPlayer::new(["frodo", "sam", "merry", "pippin"]);
    /// pp[for4::P1] = "samwise";
    /// pp[for4::P2] = "meriadoc";
    /// pp[for4::P3] = "peregrin";
    /// assert_eq!(pp[for4::P0], "frodo");
    /// assert_eq!(pp[for4::P1], "samwise");
    /// assert_eq!(pp[for4::P2], "meriadoc");
    /// assert_eq!(pp[for4::P3], "peregrin");
    /// ```
    fn index_mut(&mut self, idx: PlayerIndex<N>) -> &mut T {
        unsafe { self.data.get_unchecked_mut(idx.0) }
    }
}

/// An iterator that produces all of the player indexes of a given index type.
pub struct PlayerIndexes<const N: usize> {
    next: usize,
    back: usize,
}

impl<const N: usize> PlayerIndexes<N> {
    fn new() -> Self {
        PlayerIndexes { next: 0, back: N }
    }
}

impl<const N: usize> Iterator for PlayerIndexes<N> {
    type Item = PlayerIndex<N>;
    fn next(&mut self) -> Option<PlayerIndex<N>> {
        if self.next < self.back {
            let index = PlayerIndex(self.next);
            self.next += 1;
            Some(index)
        } else {
            None
        }
    }
}

impl<const N: usize> DoubleEndedIterator for PlayerIndexes<N> {
    fn next_back(&mut self) -> Option<PlayerIndex<N>> {
        if self.next < self.back {
            self.back -= 1;
            Some(PlayerIndex(self.back))
        } else {
            None
        }
    }
}

/// Defines indexes into a collection of type `PerPlayer<T, 1>`.
pub mod for1 {
    use super::PlayerIndex;
    /// The only player in a 1-player game.
    pub const P0: PlayerIndex<1> = PlayerIndex(0);
}

/// Defines indexes into a collection of type `PerPlayer<T, 2>` and a move type for 2-player games.
pub mod for2 {
    use super::{PerPlayer, PlayerIndex};

    /// The 1st player in a 2-player game.
    pub const P0: PlayerIndex<2> = PlayerIndex(0);
    /// The 2nd player in a 2-player game.
    pub const P1: PlayerIndex<2> = PlayerIndex(1);

    /// The *row* player in a 2-player normal form game. Same as [`P0`].
    pub const ROW: PlayerIndex<2> = P0;
    /// The *column* player in a 2-player normal form game. Same as [`P1`].
    pub const COL: PlayerIndex<2> = P1;

    /// A move in a 2-player game. This type enables defining games where each player has a
    /// different type of move.
    #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
    pub enum Move<M0, M1> {
        /// A move for player `P0`.
        P0(M0),
        /// A move for player `P1`.
        P1(M1),
    }

    /// Construct a per-player collection of moves available to each player, given a vector of
    /// moves available to each player, which may be of different types.
    pub fn per_player_moves<M0, M1>(
        p0_moves: Vec<M0>,
        p1_moves: Vec<M1>,
    ) -> PerPlayer<Vec<Move<M0, M1>>, 2> {
        PerPlayer::new([
            p0_moves.into_iter().map(|m| Move::P0(m)).collect(),
            p1_moves.into_iter().map(|m| Move::P1(m)).collect(),
        ])
    }
}

/// Defines indexes into a collection of type `PerPlayer<T, 3>` and a move type for 3-player games.
pub mod for3 {
    use super::{PerPlayer, PlayerIndex};

    /// The 1st player in a 3-player game.
    pub const P0: PlayerIndex<3> = PlayerIndex(0);
    /// The 2nd player in a 3-player game.
    pub const P1: PlayerIndex<3> = PlayerIndex(1);
    /// The 3rd player in a 3-player game.
    pub const P2: PlayerIndex<3> = PlayerIndex(2);

    /// A move in a 3-player game. This type enables defining games where each player has a
    /// different type of move.
    #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
    pub enum Move<M0, M1, M2> {
        /// A move for player `P0`.
        P0(M0),
        /// A move for player `P1`.
        P1(M1),
        /// A move for player `P2`.
        P2(M2),
    }

    /// Construct a per-player collection of moves available to each player, given a vector of
    /// moves available to each player, which may be of different types.
    pub fn per_player_moves<M0, M1, M2>(
        p0_moves: Vec<M0>,
        p1_moves: Vec<M1>,
        p2_moves: Vec<M2>,
    ) -> PerPlayer<Vec<Move<M0, M1, M2>>, 3> {
        PerPlayer::new([
            p0_moves.into_iter().map(|m| Move::P0(m)).collect(),
            p1_moves.into_iter().map(|m| Move::P1(m)).collect(),
            p2_moves.into_iter().map(|m| Move::P2(m)).collect(),
        ])
    }
}

/// Defines indexes into a collection of type `PerPlayer<T, 4>` and a move type for 4-player games.
pub mod for4 {
    use super::{PerPlayer, PlayerIndex};

    /// The 1st player in a 4-player game.
    pub const P0: PlayerIndex<4> = PlayerIndex(0);
    /// The 2nd player in a 4-player game.
    pub const P1: PlayerIndex<4> = PlayerIndex(1);
    /// The 3rd player in a 4-player game.
    pub const P2: PlayerIndex<4> = PlayerIndex(2);
    /// The 4th player in a 4-player game.
    pub const P3: PlayerIndex<4> = PlayerIndex(3);

    /// A move in a 4-player game. This type enables defining games where each player has a
    /// different type of move.
    #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
    pub enum Move<M0, M1, M2, M3> {
        /// A move for player `P0`.
        P0(M0),
        /// A move for player `P1`.
        P1(M1),
        /// A move for player `P2`.
        P2(M2),
        /// A move for player `P3`.
        P3(M3),
    }

    /// Construct a per-player collection of moves available to each player, given a vector of
    /// moves available to each player, which may be of different types.
    pub fn per_player_moves<M0, M1, M2, M3>(
        p0_moves: Vec<M0>,
        p1_moves: Vec<M1>,
        p2_moves: Vec<M2>,
        p3_moves: Vec<M3>,
    ) -> PerPlayer<Vec<Move<M0, M1, M2, M3>>, 4> {
        PerPlayer::new([
            p0_moves.into_iter().map(|m| Move::P0(m)).collect(),
            p1_moves.into_iter().map(|m| Move::P1(m)).collect(),
            p2_moves.into_iter().map(|m| Move::P2(m)).collect(),
            p3_moves.into_iter().map(|m| Move::P3(m)).collect(),
        ])
    }
}

/// Defines indexes into a collection of type `PerPlayer<T, 5>`.
pub mod for5 {
    use super::PlayerIndex;

    /// The 1st player in a 5-player game.
    pub const P0: PlayerIndex<5> = PlayerIndex(0);
    /// The 2nd player in a 5-player game.
    pub const P1: PlayerIndex<5> = PlayerIndex(1);
    /// The 3rd player in a 5-player game.
    pub const P2: PlayerIndex<5> = PlayerIndex(2);
    /// The 4th player in a 5-player game.
    pub const P3: PlayerIndex<5> = PlayerIndex(3);
    /// The 5th player in a 5-player game.
    pub const P4: PlayerIndex<5> = PlayerIndex(4);
}

/// Defines indexes into a collection of type `PerPlayer<T, 6>`.
pub mod for6 {
    use super::PlayerIndex;

    /// The 1st player in a 6-player game.
    pub const P0: PlayerIndex<6> = PlayerIndex(0);
    /// The 2nd player in a 6-player game.
    pub const P1: PlayerIndex<6> = PlayerIndex(1);
    /// The 3rd player in a 6-player game.
    pub const P2: PlayerIndex<6> = PlayerIndex(2);
    /// The 4th player in a 6-player game.
    pub const P3: PlayerIndex<6> = PlayerIndex(3);
    /// The 5th player in a 6-player game.
    pub const P4: PlayerIndex<6> = PlayerIndex(4);
    /// The 6th player in a 6-player game.
    pub const P5: PlayerIndex<6> = PlayerIndex(5);
}

/// Defines indexes into a collection of type `PerPlayer<T, 7>`.
pub mod for7 {
    use super::PlayerIndex;

    /// The 1st player in a 7-player game.
    pub const P0: PlayerIndex<7> = PlayerIndex(0);
    /// The 2nd player in a 7-player game.
    pub const P1: PlayerIndex<7> = PlayerIndex(1);
    /// The 3rd player in a 7-player game.
    pub const P2: PlayerIndex<7> = PlayerIndex(2);
    /// The 4th player in a 7-player game.
    pub const P3: PlayerIndex<7> = PlayerIndex(3);
    /// The 5th player in a 7-player game.
    pub const P4: PlayerIndex<7> = PlayerIndex(4);
    /// The 6th player in a 7-player game.
    pub const P5: PlayerIndex<7> = PlayerIndex(5);
    /// The 7th player in a 7-player game.
    pub const P6: PlayerIndex<7> = PlayerIndex(6);
}

/// Defines indexes into a collection of type `PerPlayer<T, 8>`.
pub mod for8 {
    use super::PlayerIndex;

    /// The 1st player in a 8-player game.
    pub const P0: PlayerIndex<8> = PlayerIndex(0);
    /// The 2nd player in a 8-player game.
    pub const P1: PlayerIndex<8> = PlayerIndex(1);
    /// The 3rd player in a 8-player game.
    pub const P2: PlayerIndex<8> = PlayerIndex(2);
    /// The 4th player in a 8-player game.
    pub const P3: PlayerIndex<8> = PlayerIndex(3);
    /// The 5th player in a 8-player game.
    pub const P4: PlayerIndex<8> = PlayerIndex(4);
    /// The 6th player in a 8-player game.
    pub const P5: PlayerIndex<8> = PlayerIndex(5);
    /// The 7th player in a 8-player game.
    pub const P6: PlayerIndex<8> = PlayerIndex(6);
    /// The 8th player in a 8-player game.
    pub const P7: PlayerIndex<8> = PlayerIndex(7);
}

/// Defines indexes into a collection of type `PerPlayer<T, 9>`.
pub mod for9 {
    use super::PlayerIndex;

    /// The 1st player in a 9-player game.
    pub const P0: PlayerIndex<9> = PlayerIndex(0);
    /// The 2nd player in a 9-player game.
    pub const P1: PlayerIndex<9> = PlayerIndex(1);
    /// The 3rd player in a 9-player game.
    pub const P2: PlayerIndex<9> = PlayerIndex(2);
    /// The 4th player in a 9-player game.
    pub const P3: PlayerIndex<9> = PlayerIndex(3);
    /// The 5th player in a 9-player game.
    pub const P4: PlayerIndex<9> = PlayerIndex(4);
    /// The 6th player in a 9-player game.
    pub const P5: PlayerIndex<9> = PlayerIndex(5);
    /// The 7th player in a 9-player game.
    pub const P6: PlayerIndex<9> = PlayerIndex(6);
    /// The 8th player in a 9-player game.
    pub const P7: PlayerIndex<9> = PlayerIndex(7);
    /// The 9th player in a 9-player game.
    pub const P8: PlayerIndex<9> = PlayerIndex(8);
}

/// Defines indexes into a collection of type `PerPlayer<T, 10>`.
pub mod for10 {
    use super::PlayerIndex;

    /// The 1st player in a 10-player game.
    pub const P0: PlayerIndex<10> = PlayerIndex(0);
    /// The 2nd player in a 10-player game.
    pub const P1: PlayerIndex<10> = PlayerIndex(1);
    /// The 3rd player in a 10-player game.
    pub const P2: PlayerIndex<10> = PlayerIndex(2);
    /// The 4th player in a 10-player game.
    pub const P3: PlayerIndex<10> = PlayerIndex(3);
    /// The 5th player in a 10-player game.
    pub const P4: PlayerIndex<10> = PlayerIndex(4);
    /// The 6th player in a 10-player game.
    pub const P5: PlayerIndex<10> = PlayerIndex(5);
    /// The 7th player in a 10-player game.
    pub const P6: PlayerIndex<10> = PlayerIndex(6);
    /// The 8th player in a 10-player game.
    pub const P7: PlayerIndex<10> = PlayerIndex(7);
    /// The 9th player in a 10-player game.
    pub const P8: PlayerIndex<10> = PlayerIndex(8);
    /// The 10th player in a 10-player game.
    pub const P9: PlayerIndex<10> = PlayerIndex(9);
}

/// Defines indexes into a collection of type `PerPlayer<T, 11>`.
pub mod for11 {
    use super::PlayerIndex;

    /// The 1st player in a 11-player game.
    pub const P0: PlayerIndex<11> = PlayerIndex(0);
    /// The 2nd player in a 11-player game.
    pub const P1: PlayerIndex<11> = PlayerIndex(1);
    /// The 3rd player in a 11-player game.
    pub const P2: PlayerIndex<11> = PlayerIndex(2);
    /// The 4th player in a 11-player game.
    pub const P3: PlayerIndex<11> = PlayerIndex(3);
    /// The 5th player in a 11-player game.
    pub const P4: PlayerIndex<11> = PlayerIndex(4);
    /// The 6th player in a 11-player game.
    pub const P5: PlayerIndex<11> = PlayerIndex(5);
    /// The 7th player in a 11-player game.
    pub const P6: PlayerIndex<11> = PlayerIndex(6);
    /// The 8th player in a 11-player game.
    pub const P7: PlayerIndex<11> = PlayerIndex(7);
    /// The 9th player in a 11-player game.
    pub const P8: PlayerIndex<11> = PlayerIndex(8);
    /// The 10th player in a 11-player game.
    pub const P9: PlayerIndex<11> = PlayerIndex(9);
    /// The 11th player in a 11-player game.
    pub const P10: PlayerIndex<11> = PlayerIndex(10);
}

/// Defines indexes into a collection of type `PerPlayer<T, 12>`.
pub mod for12 {
    use super::PlayerIndex;

    /// The 1st player in a 12-player game.
    pub const P0: PlayerIndex<12> = PlayerIndex(0);
    /// The 2nd player in a 12-player game.
    pub const P1: PlayerIndex<12> = PlayerIndex(1);
    /// The 3rd player in a 12-player game.
    pub const P2: PlayerIndex<12> = PlayerIndex(2);
    /// The 4th player in a 12-player game.
    pub const P3: PlayerIndex<12> = PlayerIndex(3);
    /// The 5th player in a 12-player game.
    pub const P4: PlayerIndex<12> = PlayerIndex(4);
    /// The 6th player in a 12-player game.
    pub const P5: PlayerIndex<12> = PlayerIndex(5);
    /// The 7th player in a 12-player game.
    pub const P6: PlayerIndex<12> = PlayerIndex(6);
    /// The 8th player in a 12-player game.
    pub const P7: PlayerIndex<12> = PlayerIndex(7);
    /// The 9th player in a 12-player game.
    pub const P8: PlayerIndex<12> = PlayerIndex(8);
    /// The 10th player in a 12-player game.
    pub const P9: PlayerIndex<12> = PlayerIndex(9);
    /// The 11th player in a 12-player game.
    pub const P10: PlayerIndex<12> = PlayerIndex(10);
    /// The 12th player in a 12-player game.
    pub const P11: PlayerIndex<12> = PlayerIndex(11);
}

/// Defines indexes into a collection of type `PerPlayer<T, 13>`.
pub mod for13 {
    use super::PlayerIndex;

    /// The 1st player in a 13-player game.
    pub const P0: PlayerIndex<13> = PlayerIndex(0);
    /// The 2nd player in a 13-player game.
    pub const P1: PlayerIndex<13> = PlayerIndex(1);
    /// The 3rd player in a 13-player game.
    pub const P2: PlayerIndex<13> = PlayerIndex(2);
    /// The 4th player in a 13-player game.
    pub const P3: PlayerIndex<13> = PlayerIndex(3);
    /// The 5th player in a 13-player game.
    pub const P4: PlayerIndex<13> = PlayerIndex(4);
    /// The 6th player in a 13-player game.
    pub const P5: PlayerIndex<13> = PlayerIndex(5);
    /// The 7th player in a 13-player game.
    pub const P6: PlayerIndex<13> = PlayerIndex(6);
    /// The 8th player in a 13-player game.
    pub const P7: PlayerIndex<13> = PlayerIndex(7);
    /// The 9th player in a 13-player game.
    pub const P8: PlayerIndex<13> = PlayerIndex(8);
    /// The 10th player in a 13-player game.
    pub const P9: PlayerIndex<13> = PlayerIndex(9);
    /// The 11th player in a 13-player game.
    pub const P10: PlayerIndex<13> = PlayerIndex(10);
    /// The 12th player in a 13-player game.
    pub const P11: PlayerIndex<13> = PlayerIndex(11);
    /// The 13th player in a 13-player game.
    pub const P12: PlayerIndex<13> = PlayerIndex(12);
}

/// Defines indexes into a collection of type `PerPlayer<T, 14>`.
pub mod for14 {
    use super::PlayerIndex;

    /// The 1st player in a 14-player game.
    pub const P0: PlayerIndex<14> = PlayerIndex(0);
    /// The 2nd player in a 14-player game.
    pub const P1: PlayerIndex<14> = PlayerIndex(1);
    /// The 3rd player in a 14-player game.
    pub const P2: PlayerIndex<14> = PlayerIndex(2);
    /// The 4th player in a 14-player game.
    pub const P3: PlayerIndex<14> = PlayerIndex(3);
    /// The 5th player in a 14-player game.
    pub const P4: PlayerIndex<14> = PlayerIndex(4);
    /// The 6th player in a 14-player game.
    pub const P5: PlayerIndex<14> = PlayerIndex(5);
    /// The 7th player in a 14-player game.
    pub const P6: PlayerIndex<14> = PlayerIndex(6);
    /// The 8th player in a 14-player game.
    pub const P7: PlayerIndex<14> = PlayerIndex(7);
    /// The 9th player in a 14-player game.
    pub const P8: PlayerIndex<14> = PlayerIndex(8);
    /// The 10th player in a 14-player game.
    pub const P9: PlayerIndex<14> = PlayerIndex(9);
    /// The 11th player in a 14-player game.
    pub const P10: PlayerIndex<14> = PlayerIndex(10);
    /// The 12th player in a 14-player game.
    pub const P11: PlayerIndex<14> = PlayerIndex(11);
    /// The 13th player in a 14-player game.
    pub const P12: PlayerIndex<14> = PlayerIndex(12);
    /// The 14th player in a 14-player game.
    pub const P13: PlayerIndex<14> = PlayerIndex(13);
}

/// Defines indexes into a collection of type `PerPlayer<T, 15>`.
pub mod for15 {
    use super::PlayerIndex;

    /// The 1st player in a 15-player game.
    pub const P0: PlayerIndex<15> = PlayerIndex(0);
    /// The 2nd player in a 15-player game.
    pub const P1: PlayerIndex<15> = PlayerIndex(1);
    /// The 3rd player in a 15-player game.
    pub const P2: PlayerIndex<15> = PlayerIndex(2);
    /// The 4th player in a 15-player game.
    pub const P3: PlayerIndex<15> = PlayerIndex(3);
    /// The 5th player in a 15-player game.
    pub const P4: PlayerIndex<15> = PlayerIndex(4);
    /// The 6th player in a 15-player game.
    pub const P5: PlayerIndex<15> = PlayerIndex(5);
    /// The 7th player in a 15-player game.
    pub const P6: PlayerIndex<15> = PlayerIndex(6);
    /// The 8th player in a 15-player game.
    pub const P7: PlayerIndex<15> = PlayerIndex(7);
    /// The 9th player in a 15-player game.
    pub const P8: PlayerIndex<15> = PlayerIndex(8);
    /// The 10th player in a 15-player game.
    pub const P9: PlayerIndex<15> = PlayerIndex(9);
    /// The 11th player in a 15-player game.
    pub const P10: PlayerIndex<15> = PlayerIndex(10);
    /// The 12th player in a 15-player game.
    pub const P11: PlayerIndex<15> = PlayerIndex(11);
    /// The 13th player in a 15-player game.
    pub const P12: PlayerIndex<15> = PlayerIndex(12);
    /// The 14th player in a 15-player game.
    pub const P13: PlayerIndex<15> = PlayerIndex(13);
    /// The 15th player in a 15-player game.
    pub const P14: PlayerIndex<15> = PlayerIndex(14);
}

/// Defines indexes into a collection of type `PerPlayer<T, 16>`.
pub mod for16 {
    use super::PlayerIndex;

    /// The 1st player in a 16-player game.
    pub const P0: PlayerIndex<16> = PlayerIndex(0);
    /// The 2nd player in a 16-player game.
    pub const P1: PlayerIndex<16> = PlayerIndex(1);
    /// The 3rd player in a 16-player game.
    pub const P2: PlayerIndex<16> = PlayerIndex(2);
    /// The 4th player in a 16-player game.
    pub const P3: PlayerIndex<16> = PlayerIndex(3);
    /// The 5th player in a 16-player game.
    pub const P4: PlayerIndex<16> = PlayerIndex(4);
    /// The 6th player in a 16-player game.
    pub const P5: PlayerIndex<16> = PlayerIndex(5);
    /// The 7th player in a 16-player game.
    pub const P6: PlayerIndex<16> = PlayerIndex(6);
    /// The 8th player in a 16-player game.
    pub const P7: PlayerIndex<16> = PlayerIndex(7);
    /// The 9th player in a 16-player game.
    pub const P8: PlayerIndex<16> = PlayerIndex(8);
    /// The 10th player in a 16-player game.
    pub const P9: PlayerIndex<16> = PlayerIndex(9);
    /// The 11th player in a 16-player game.
    pub const P10: PlayerIndex<16> = PlayerIndex(10);
    /// The 12th player in a 16-player game.
    pub const P11: PlayerIndex<16> = PlayerIndex(11);
    /// The 13th player in a 16-player game.
    pub const P12: PlayerIndex<16> = PlayerIndex(12);
    /// The 14th player in a 16-player game.
    pub const P13: PlayerIndex<16> = PlayerIndex(13);
    /// The 15th player in a 16-player game.
    pub const P14: PlayerIndex<16> = PlayerIndex(14);
    /// The 16th player in a 16-player game.
    pub const P15: PlayerIndex<16> = PlayerIndex(15);
}
