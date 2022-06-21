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
/// use tft::core::PerPlayer;
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
/// use tft::core::PlayerIndex;
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
/// use tft::core::{PerPlayer, PlayerIndex};
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
/// use tft::core::{for3, PerPlayer};
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
    /// use tft::core::{for5, PerPlayer, PlayerIndex};
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
    pub fn generate(gen_elem: impl Fn(PlayerIndex<N>) -> T) -> Self {
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
    /// use tft::core::PerPlayer;
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
            None
        }
    }

    /// Get a mutable reference to the element corresponding to the `i`th player in the game.
    /// Returns `None` if the index is out of range.
    ///
    /// # Examples
    /// ```
    /// use tft::core::PerPlayer;
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
            None
        }
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
        (&self.data).iter()
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a mut PerPlayer<T, N> {
    type Item = <&'a mut [T; N] as IntoIterator>::Item;
    type IntoIter = <&'a mut [T; N] as IntoIterator>::IntoIter;
    fn into_iter(self) -> <&'a mut [T; N] as IntoIterator>::IntoIter {
        (&mut self.data).iter_mut()
    }
}

/// An index into a [`PerPlayer`] collection that is guaranteed to be in-range for a game with `N`
/// players.
///
/// This type is used in the implementations of the [`Index`] and [`IndexMut`] traits and ensures
/// that their operations will not fail.
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
    /// use tft::core::{for2, for8, PlayerIndex};
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
            None
        }
    }

    /// Get an iterator that iterates over all player indexes of a given type.
    ///
    /// # Examples
    /// ```
    /// use tft::core::{for3, for5, PlayerIndex};
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
        PlayerIndexes { next: 0 }
    }
}

impl<const N: usize> From<PlayerIndex<N>> for usize {
    fn from(index: PlayerIndex<N>) -> usize {
        index.0
    }
}

impl<T, const N: usize> Index<PlayerIndex<N>> for PerPlayer<T, N> {
    type Output = T;
    /// Index into a `PerPlayer` collection. This operation is statically guaranteed not to fail.
    ///
    /// # Examples
    /// ```
    /// use tft::core::{for4, PerPlayer};
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
    /// use tft::core::{for4, PerPlayer};
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
}

impl<const N: usize> Iterator for PlayerIndexes<N> {
    type Item = PlayerIndex<N>;
    fn next(&mut self) -> Option<PlayerIndex<N>> {
        if self.next < N {
            let index = PlayerIndex(self.next);
            self.next += 1;
            Some(index)
        } else {
            None
        }
    }
}

/// Defines indexes into a collection of type `PerPlayer<T, 1>`.
pub mod for1 {
    use super::PlayerIndex;
    pub const P0: PlayerIndex<1> = PlayerIndex(0);
}

/// Defines indexes into a collection of type `PerPlayer<T, 2>`.
pub mod for2 {
    use super::PlayerIndex;
    pub const P0: PlayerIndex<2> = PlayerIndex(0);
    pub const P1: PlayerIndex<2> = PlayerIndex(1);
}

/// Defines indexes into a collection of type `PerPlayer<T, 3>`.
pub mod for3 {
    use super::PlayerIndex;
    pub const P0: PlayerIndex<3> = PlayerIndex(0);
    pub const P1: PlayerIndex<3> = PlayerIndex(1);
    pub const P2: PlayerIndex<3> = PlayerIndex(2);
}

/// Defines indexes into a collection of type `PerPlayer<T, 4>`.
pub mod for4 {
    use super::PlayerIndex;
    pub const P0: PlayerIndex<4> = PlayerIndex(0);
    pub const P1: PlayerIndex<4> = PlayerIndex(1);
    pub const P2: PlayerIndex<4> = PlayerIndex(2);
    pub const P3: PlayerIndex<4> = PlayerIndex(3);
}

/// Defines indexes into a collection of type `PerPlayer<T, 5>`.
pub mod for5 {
    use super::PlayerIndex;
    pub const P0: PlayerIndex<5> = PlayerIndex(0);
    pub const P1: PlayerIndex<5> = PlayerIndex(1);
    pub const P2: PlayerIndex<5> = PlayerIndex(2);
    pub const P3: PlayerIndex<5> = PlayerIndex(3);
    pub const P4: PlayerIndex<5> = PlayerIndex(4);
}

/// Defines indexes into a collection of type `PerPlayer<T, 6>`.
pub mod for6 {
    use super::PlayerIndex;
    pub const P0: PlayerIndex<6> = PlayerIndex(0);
    pub const P1: PlayerIndex<6> = PlayerIndex(1);
    pub const P2: PlayerIndex<6> = PlayerIndex(2);
    pub const P3: PlayerIndex<6> = PlayerIndex(3);
    pub const P4: PlayerIndex<6> = PlayerIndex(4);
    pub const P5: PlayerIndex<6> = PlayerIndex(5);
}

/// Defines indexes into a collection of type `PerPlayer<T, 7>`.
pub mod for7 {
    use super::PlayerIndex;
    pub const P0: PlayerIndex<7> = PlayerIndex(0);
    pub const P1: PlayerIndex<7> = PlayerIndex(1);
    pub const P2: PlayerIndex<7> = PlayerIndex(2);
    pub const P3: PlayerIndex<7> = PlayerIndex(3);
    pub const P4: PlayerIndex<7> = PlayerIndex(4);
    pub const P5: PlayerIndex<7> = PlayerIndex(5);
    pub const P6: PlayerIndex<7> = PlayerIndex(6);
}

/// Defines indexes into a collection of type `PerPlayer<T, 8>`.
pub mod for8 {
    use super::PlayerIndex;
    pub const P0: PlayerIndex<8> = PlayerIndex(0);
    pub const P1: PlayerIndex<8> = PlayerIndex(1);
    pub const P2: PlayerIndex<8> = PlayerIndex(2);
    pub const P3: PlayerIndex<8> = PlayerIndex(3);
    pub const P4: PlayerIndex<8> = PlayerIndex(4);
    pub const P5: PlayerIndex<8> = PlayerIndex(5);
    pub const P6: PlayerIndex<8> = PlayerIndex(6);
    pub const P7: PlayerIndex<8> = PlayerIndex(7);
}

/// Defines indexes into a collection of type `PerPlayer<T, 9>`.
pub mod for9 {
    use super::PlayerIndex;
    pub const P0: PlayerIndex<9> = PlayerIndex(0);
    pub const P1: PlayerIndex<9> = PlayerIndex(1);
    pub const P2: PlayerIndex<9> = PlayerIndex(2);
    pub const P3: PlayerIndex<9> = PlayerIndex(3);
    pub const P4: PlayerIndex<9> = PlayerIndex(4);
    pub const P5: PlayerIndex<9> = PlayerIndex(5);
    pub const P6: PlayerIndex<9> = PlayerIndex(6);
    pub const P7: PlayerIndex<9> = PlayerIndex(7);
    pub const P8: PlayerIndex<9> = PlayerIndex(8);
}

/// Defines indexes into a collection of type `PerPlayer<T, 10>`.
pub mod for10 {
    use super::PlayerIndex;
    pub const P0: PlayerIndex<10> = PlayerIndex(0);
    pub const P1: PlayerIndex<10> = PlayerIndex(1);
    pub const P2: PlayerIndex<10> = PlayerIndex(2);
    pub const P3: PlayerIndex<10> = PlayerIndex(3);
    pub const P4: PlayerIndex<10> = PlayerIndex(4);
    pub const P5: PlayerIndex<10> = PlayerIndex(5);
    pub const P6: PlayerIndex<10> = PlayerIndex(6);
    pub const P7: PlayerIndex<10> = PlayerIndex(7);
    pub const P8: PlayerIndex<10> = PlayerIndex(8);
    pub const P9: PlayerIndex<10> = PlayerIndex(9);
}

/// Defines indexes into a collection of type `PerPlayer<T, 11>`.
pub mod for11 {
    use super::PlayerIndex;
    pub const P0: PlayerIndex<11> = PlayerIndex(0);
    pub const P1: PlayerIndex<11> = PlayerIndex(1);
    pub const P2: PlayerIndex<11> = PlayerIndex(2);
    pub const P3: PlayerIndex<11> = PlayerIndex(3);
    pub const P4: PlayerIndex<11> = PlayerIndex(4);
    pub const P5: PlayerIndex<11> = PlayerIndex(5);
    pub const P6: PlayerIndex<11> = PlayerIndex(6);
    pub const P7: PlayerIndex<11> = PlayerIndex(7);
    pub const P8: PlayerIndex<11> = PlayerIndex(8);
    pub const P9: PlayerIndex<11> = PlayerIndex(9);
    pub const P10: PlayerIndex<11> = PlayerIndex(10);
}

/// Defines indexes into a collection of type `PerPlayer<T, 12>`.
pub mod for12 {
    use super::PlayerIndex;
    pub const P0: PlayerIndex<12> = PlayerIndex(0);
    pub const P1: PlayerIndex<12> = PlayerIndex(1);
    pub const P2: PlayerIndex<12> = PlayerIndex(2);
    pub const P3: PlayerIndex<12> = PlayerIndex(3);
    pub const P4: PlayerIndex<12> = PlayerIndex(4);
    pub const P5: PlayerIndex<12> = PlayerIndex(5);
    pub const P6: PlayerIndex<12> = PlayerIndex(6);
    pub const P7: PlayerIndex<12> = PlayerIndex(7);
    pub const P8: PlayerIndex<12> = PlayerIndex(8);
    pub const P9: PlayerIndex<12> = PlayerIndex(9);
    pub const P10: PlayerIndex<12> = PlayerIndex(10);
    pub const P11: PlayerIndex<12> = PlayerIndex(11);
}

/// Defines indexes into a collection of type `PerPlayer<T, 13>`.
pub mod for13 {
    use super::PlayerIndex;
    pub const P0: PlayerIndex<13> = PlayerIndex(0);
    pub const P1: PlayerIndex<13> = PlayerIndex(1);
    pub const P2: PlayerIndex<13> = PlayerIndex(2);
    pub const P3: PlayerIndex<13> = PlayerIndex(3);
    pub const P4: PlayerIndex<13> = PlayerIndex(4);
    pub const P5: PlayerIndex<13> = PlayerIndex(5);
    pub const P6: PlayerIndex<13> = PlayerIndex(6);
    pub const P7: PlayerIndex<13> = PlayerIndex(7);
    pub const P8: PlayerIndex<13> = PlayerIndex(8);
    pub const P9: PlayerIndex<13> = PlayerIndex(9);
    pub const P10: PlayerIndex<13> = PlayerIndex(10);
    pub const P11: PlayerIndex<13> = PlayerIndex(11);
    pub const P12: PlayerIndex<13> = PlayerIndex(12);
}

/// Defines indexes into a collection of type `PerPlayer<T, 14>`.
pub mod for14 {
    use super::PlayerIndex;
    pub const P0: PlayerIndex<14> = PlayerIndex(0);
    pub const P1: PlayerIndex<14> = PlayerIndex(1);
    pub const P2: PlayerIndex<14> = PlayerIndex(2);
    pub const P3: PlayerIndex<14> = PlayerIndex(3);
    pub const P4: PlayerIndex<14> = PlayerIndex(4);
    pub const P5: PlayerIndex<14> = PlayerIndex(5);
    pub const P6: PlayerIndex<14> = PlayerIndex(6);
    pub const P7: PlayerIndex<14> = PlayerIndex(7);
    pub const P8: PlayerIndex<14> = PlayerIndex(8);
    pub const P9: PlayerIndex<14> = PlayerIndex(9);
    pub const P10: PlayerIndex<14> = PlayerIndex(10);
    pub const P11: PlayerIndex<14> = PlayerIndex(11);
    pub const P12: PlayerIndex<14> = PlayerIndex(12);
    pub const P13: PlayerIndex<14> = PlayerIndex(13);
}

/// Defines indexes into a collection of type `PerPlayer<T, 15>`.
pub mod for15 {
    use super::PlayerIndex;
    pub const P0: PlayerIndex<15> = PlayerIndex(0);
    pub const P1: PlayerIndex<15> = PlayerIndex(1);
    pub const P2: PlayerIndex<15> = PlayerIndex(2);
    pub const P3: PlayerIndex<15> = PlayerIndex(3);
    pub const P4: PlayerIndex<15> = PlayerIndex(4);
    pub const P5: PlayerIndex<15> = PlayerIndex(5);
    pub const P6: PlayerIndex<15> = PlayerIndex(6);
    pub const P7: PlayerIndex<15> = PlayerIndex(7);
    pub const P8: PlayerIndex<15> = PlayerIndex(8);
    pub const P9: PlayerIndex<15> = PlayerIndex(9);
    pub const P10: PlayerIndex<15> = PlayerIndex(10);
    pub const P11: PlayerIndex<15> = PlayerIndex(11);
    pub const P12: PlayerIndex<15> = PlayerIndex(12);
    pub const P13: PlayerIndex<15> = PlayerIndex(13);
    pub const P14: PlayerIndex<15> = PlayerIndex(14);
}

/// Defines indexes into a collection of type `PerPlayer<T, 16>`.
pub mod for16 {
    use super::PlayerIndex;
    pub const P0: PlayerIndex<16> = PlayerIndex(0);
    pub const P1: PlayerIndex<16> = PlayerIndex(1);
    pub const P2: PlayerIndex<16> = PlayerIndex(2);
    pub const P3: PlayerIndex<16> = PlayerIndex(3);
    pub const P4: PlayerIndex<16> = PlayerIndex(4);
    pub const P5: PlayerIndex<16> = PlayerIndex(5);
    pub const P6: PlayerIndex<16> = PlayerIndex(6);
    pub const P7: PlayerIndex<16> = PlayerIndex(7);
    pub const P8: PlayerIndex<16> = PlayerIndex(8);
    pub const P9: PlayerIndex<16> = PlayerIndex(9);
    pub const P10: PlayerIndex<16> = PlayerIndex(10);
    pub const P11: PlayerIndex<16> = PlayerIndex(11);
    pub const P12: PlayerIndex<16> = PlayerIndex(12);
    pub const P13: PlayerIndex<16> = PlayerIndex(13);
    pub const P14: PlayerIndex<16> = PlayerIndex(14);
    pub const P15: PlayerIndex<16> = PlayerIndex(15);
}
