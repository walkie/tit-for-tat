//! This module defines the [`PerPlayer`] collection type that stores one element for each player
//! in a game.

use derive_more::{AsMut, AsRef};
use std::iter::IntoIterator;
use std::ops::{Index, IndexMut};

/// A collection that stores one element corresponding to each player in a game.
///
/// The type is parameterized by the type of elements `T` and the number of players in the game `N`.
/// For example, the type `PerPlayer<f64, 3>` contains exactly three `f64` values, one for each
/// player in a three-player game.
///
/// The ["const generic"](https://blog.rust-lang.org/2021/02/26/const-generics-mvp-beta.html)
/// argument `N` is used to statically ensure that a [`PerPlayer`] collection contains the correct
/// number of elements for a given game, and to provide statically checked indexing into
/// `PerPlayer` collections.
///
/// # Dynamically checked indexes into a `PerPlayer` collection
///
/// The [`for_player`](PerPlayer::for_player) and [`for_player_mut`](PerPlayer::for_player_mut)
/// methods allow indexing into a `PerPlayer` collection with plain `usize` indexes. They return
/// references wrapped in an [`Option`] type, which may be `None` if the index is too large for the
/// number of players in the game.
///
/// ```
/// use game_theory::per_player::PerPlayer;
///
/// let mut pp = PerPlayer::new(["klaatu", "barada", "nikto"]);
/// assert_eq!(pp.for_player(0).copied(), Some("klaatu"));
/// assert_eq!(pp.for_player(1).copied(), Some("barada"));
/// assert_eq!(pp.for_player(2).copied(), Some("nikto"));
/// assert_eq!(pp.for_player(3).copied(), None);
///
/// *pp.for_player_mut(0).unwrap() = "gort";
/// assert_eq!(pp.for_player(0).copied(), Some("gort"));
/// ```
///
/// # Statically checked indexes into a `PerPlayer` collection
///
/// The [`Index`] and [`IndexMut`] traits are implemented for `PerPlayer` collections with indexes
/// of type [`PlayerIdx`]. An index of type `PlayerIdx<N>` is guaranteed to be in-range for a
/// collection of type `PerPlayer<T, N>`, that is, indexing operations into a `PerPlayer`
/// collection are guaranteed not to fail due to an index out-of-bounds error.
///
/// Indexes can be constructed dynamically using the [`PlayerIdx::new`] constructor. Although the
/// *indexing operation* cannot fail, *constructing an index* may fail if the index is out of
/// bounds, in which case the constructor will return `None`.
///
/// ```
/// use game_theory::per_player::PlayerIdx;
///
/// assert!(PlayerIdx::<3>::new(0).is_some());
/// assert!(PlayerIdx::<3>::new(1).is_some());
/// assert!(PlayerIdx::<3>::new(2).is_some());
/// assert!(PlayerIdx::<3>::new(3).is_none());
/// ```
///
/// When constructing indexes, often the value of `N` can be inferred from the type of the
/// `PerPlayer` collection it is used to index into.
///
/// ```
/// use game_theory::per_player::{PerPlayer, PlayerIdx};
///
/// let mut pp = PerPlayer::new(["klaatu", "barada", "nikto"]);
/// let p0 = PlayerIdx::new(0).unwrap();
/// let p1 = PlayerIdx::new(1).unwrap();
/// let p2 = PlayerIdx::new(2).unwrap();
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
/// use game_theory::per_player::{for3, PerPlayer};
///
/// let mut pp = PerPlayer::new(["klaatu", "barada", "nikto"]);
/// assert_eq!(pp[for3::P0], "klaatu");
/// assert_eq!(pp[for3::P1], "barada");
/// assert_eq!(pp[for3::P2], "nikto");
///
/// pp[for3::P0] = "gort";
/// assert_eq!(pp[for3::P0], "gort");
/// ```
#[derive(Clone, Debug, Eq, PartialEq, AsMut, AsRef)]
pub struct PerPlayer<T, const N: usize> {
    data: [T; N],
}

impl<T, const N: usize> PerPlayer<T, N> {
    /// Create a new [`PerPlayer`] collection from the given array.
    pub fn new(data: [T; N]) -> Self {
        PerPlayer { data }
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
    /// use game_theory::per_player::PerPlayer;
    ///
    /// let mut pp = PerPlayer::new(["frodo", "sam", "merry", "pippin"]);
    /// assert_eq!(pp.for_player(0).copied(), Some("frodo"));
    /// assert_eq!(pp.for_player(1).copied(), Some("sam"));
    /// assert_eq!(pp.for_player(2).copied(), Some("merry"));
    /// assert_eq!(pp.for_player(3).copied(), Some("pippin"));
    /// assert_eq!(pp.for_player(4).copied(), None);
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
    /// use game_theory::per_player::PerPlayer;
    ///
    /// let mut pp = PerPlayer::new(["frodo", "sam", "merry", "pippin"]);
    /// *pp.for_player_mut(1).unwrap() = "samwise";
    /// *pp.for_player_mut(2).unwrap() = "meriadoc";
    /// *pp.for_player_mut(3).unwrap() = "peregrin";
    /// assert_eq!(pp.for_player(0).copied(), Some("frodo"));
    /// assert_eq!(pp.for_player(1).copied(), Some("samwise"));
    /// assert_eq!(pp.for_player(2).copied(), Some("meriadoc"));
    /// assert_eq!(pp.for_player(3).copied(), Some("peregrin"));
    /// assert_eq!(pp.for_player(4).copied(), None);
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlayerIdx<const N: usize>(usize);

impl<const N: usize> PlayerIdx<N> {
    /// Construct a new index into a [`PerPlayer`] collection. Returns `None` if the provided index
    /// value is out-of-range for the number of players in the game.
    ///
    /// Predefined indexes for games of up to 16 players are defined in the `forN` modules.
    ///
    /// # Examples
    /// ```
    /// use game_theory::per_player::{for2, for8, PlayerIdx};
    ///
    /// let p0_opt = PlayerIdx::<2>::new(0);
    /// let p1_opt = PlayerIdx::<2>::new(1);
    /// let p2_opt = PlayerIdx::<2>::new(2);
    ///
    /// assert!(p0_opt.is_some());
    /// assert!(p1_opt.is_some());
    /// assert!(p2_opt.is_none());
    ///
    /// assert_eq!(p0_opt.unwrap(), for2::P0);
    /// assert_eq!(p1_opt.unwrap(), for2::P1);
    ///
    /// assert_eq!(PlayerIdx::<8>::new(3).unwrap(), for8::P3);
    /// assert_eq!(PlayerIdx::<8>::new(5).unwrap(), for8::P5);
    /// ```
    pub fn new(index: usize) -> Option<Self> {
        if index < N {
            Some(PlayerIdx(index))
        } else {
            None
        }
    }
}

impl<T, const N: usize> Index<PlayerIdx<N>> for PerPlayer<T, N> {
    type Output = T;
    /// Index into a `PerPlayer` collection. This operation is statically guaranteed not to fail.
    ///
    /// # Examples
    /// ```
    /// use game_theory::per_player::{for4, PerPlayer};
    ///
    /// let mut pp = PerPlayer::new(["frodo", "sam", "merry", "pippin"]);
    /// assert_eq!(pp[for4::P0], "frodo");
    /// assert_eq!(pp[for4::P1], "sam");
    /// assert_eq!(pp[for4::P2], "merry");
    /// assert_eq!(pp[for4::P3], "pippin");
    /// ```
    fn index(&self, idx: PlayerIdx<N>) -> &T {
        unsafe { self.data.get_unchecked(idx.0) }
    }
}

impl<T, const N: usize> IndexMut<PlayerIdx<N>> for PerPlayer<T, N> {
    /// Index into a `PerPlayer` collection in a mutable context. This operation is statically
    /// guaranteed not to fail.
    ///
    /// # Examples
    /// ```
    /// use game_theory::per_player::{for4, PerPlayer};
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
    fn index_mut(&mut self, idx: PlayerIdx<N>) -> &mut T {
        unsafe { self.data.get_unchecked_mut(idx.0) }
    }
}

/// Defines indexes into a collection of type `PerPlayer<T, 1>`.
pub mod for1 {
    use super::PlayerIdx;
    pub const P0: PlayerIdx<1> = PlayerIdx(0);
}

/// Defines indexes into a collection of type `PerPlayer<T, 2>`.
pub mod for2 {
    use super::PlayerIdx;
    pub const P0: PlayerIdx<2> = PlayerIdx(0);
    pub const P1: PlayerIdx<2> = PlayerIdx(1);
}

/// Defines indexes into a collection of type `PerPlayer<T, 3>`.
pub mod for3 {
    use super::PlayerIdx;
    pub const P0: PlayerIdx<3> = PlayerIdx(0);
    pub const P1: PlayerIdx<3> = PlayerIdx(1);
    pub const P2: PlayerIdx<3> = PlayerIdx(2);
}

/// Defines indexes into a collection of type `PerPlayer<T, 4>`.
pub mod for4 {
    use super::PlayerIdx;
    pub const P0: PlayerIdx<4> = PlayerIdx(0);
    pub const P1: PlayerIdx<4> = PlayerIdx(1);
    pub const P2: PlayerIdx<4> = PlayerIdx(2);
    pub const P3: PlayerIdx<4> = PlayerIdx(3);
}

/// Defines indexes into a collection of type `PerPlayer<T, 5>`.
pub mod for5 {
    use super::PlayerIdx;
    pub const P0: PlayerIdx<5> = PlayerIdx(0);
    pub const P1: PlayerIdx<5> = PlayerIdx(1);
    pub const P2: PlayerIdx<5> = PlayerIdx(2);
    pub const P3: PlayerIdx<5> = PlayerIdx(3);
    pub const P4: PlayerIdx<5> = PlayerIdx(4);
}

/// Defines indexes into a collection of type `PerPlayer<T, 6>`.
pub mod for6 {
    use super::PlayerIdx;
    pub const P0: PlayerIdx<6> = PlayerIdx(0);
    pub const P1: PlayerIdx<6> = PlayerIdx(1);
    pub const P2: PlayerIdx<6> = PlayerIdx(2);
    pub const P3: PlayerIdx<6> = PlayerIdx(3);
    pub const P4: PlayerIdx<6> = PlayerIdx(4);
    pub const P5: PlayerIdx<6> = PlayerIdx(5);
}

/// Defines indexes into a collection of type `PerPlayer<T, 7>`.
pub mod for7 {
    use super::PlayerIdx;
    pub const P0: PlayerIdx<7> = PlayerIdx(0);
    pub const P1: PlayerIdx<7> = PlayerIdx(1);
    pub const P2: PlayerIdx<7> = PlayerIdx(2);
    pub const P3: PlayerIdx<7> = PlayerIdx(3);
    pub const P4: PlayerIdx<7> = PlayerIdx(4);
    pub const P5: PlayerIdx<7> = PlayerIdx(5);
    pub const P6: PlayerIdx<7> = PlayerIdx(6);
}

/// Defines indexes into a collection of type `PerPlayer<T, 8>`.
pub mod for8 {
    use super::PlayerIdx;
    pub const P0: PlayerIdx<8> = PlayerIdx(0);
    pub const P1: PlayerIdx<8> = PlayerIdx(1);
    pub const P2: PlayerIdx<8> = PlayerIdx(2);
    pub const P3: PlayerIdx<8> = PlayerIdx(3);
    pub const P4: PlayerIdx<8> = PlayerIdx(4);
    pub const P5: PlayerIdx<8> = PlayerIdx(5);
    pub const P6: PlayerIdx<8> = PlayerIdx(6);
    pub const P7: PlayerIdx<8> = PlayerIdx(7);
}

/// Defines indexes into a collection of type `PerPlayer<T, 9>`.
pub mod for9 {
    use super::PlayerIdx;
    pub const P0: PlayerIdx<9> = PlayerIdx(0);
    pub const P1: PlayerIdx<9> = PlayerIdx(1);
    pub const P2: PlayerIdx<9> = PlayerIdx(2);
    pub const P3: PlayerIdx<9> = PlayerIdx(3);
    pub const P4: PlayerIdx<9> = PlayerIdx(4);
    pub const P5: PlayerIdx<9> = PlayerIdx(5);
    pub const P6: PlayerIdx<9> = PlayerIdx(6);
    pub const P7: PlayerIdx<9> = PlayerIdx(7);
    pub const P8: PlayerIdx<9> = PlayerIdx(8);
}

/// Defines indexes into a collection of type `PerPlayer<T, 10>`.
pub mod for10 {
    use super::PlayerIdx;
    pub const P0: PlayerIdx<10> = PlayerIdx(0);
    pub const P1: PlayerIdx<10> = PlayerIdx(1);
    pub const P2: PlayerIdx<10> = PlayerIdx(2);
    pub const P3: PlayerIdx<10> = PlayerIdx(3);
    pub const P4: PlayerIdx<10> = PlayerIdx(4);
    pub const P5: PlayerIdx<10> = PlayerIdx(5);
    pub const P6: PlayerIdx<10> = PlayerIdx(6);
    pub const P7: PlayerIdx<10> = PlayerIdx(7);
    pub const P8: PlayerIdx<10> = PlayerIdx(8);
    pub const P9: PlayerIdx<10> = PlayerIdx(9);
}

/// Defines indexes into a collection of type `PerPlayer<T, 11>`.
pub mod for11 {
    use super::PlayerIdx;
    pub const P0: PlayerIdx<11> = PlayerIdx(0);
    pub const P1: PlayerIdx<11> = PlayerIdx(1);
    pub const P2: PlayerIdx<11> = PlayerIdx(2);
    pub const P3: PlayerIdx<11> = PlayerIdx(3);
    pub const P4: PlayerIdx<11> = PlayerIdx(4);
    pub const P5: PlayerIdx<11> = PlayerIdx(5);
    pub const P6: PlayerIdx<11> = PlayerIdx(6);
    pub const P7: PlayerIdx<11> = PlayerIdx(7);
    pub const P8: PlayerIdx<11> = PlayerIdx(8);
    pub const P9: PlayerIdx<11> = PlayerIdx(9);
    pub const P10: PlayerIdx<11> = PlayerIdx(10);
}

/// Defines indexes into a collection of type `PerPlayer<T, 12>`.
pub mod for12 {
    use super::PlayerIdx;
    pub const P0: PlayerIdx<12> = PlayerIdx(0);
    pub const P1: PlayerIdx<12> = PlayerIdx(1);
    pub const P2: PlayerIdx<12> = PlayerIdx(2);
    pub const P3: PlayerIdx<12> = PlayerIdx(3);
    pub const P4: PlayerIdx<12> = PlayerIdx(4);
    pub const P5: PlayerIdx<12> = PlayerIdx(5);
    pub const P6: PlayerIdx<12> = PlayerIdx(6);
    pub const P7: PlayerIdx<12> = PlayerIdx(7);
    pub const P8: PlayerIdx<12> = PlayerIdx(8);
    pub const P9: PlayerIdx<12> = PlayerIdx(9);
    pub const P10: PlayerIdx<12> = PlayerIdx(10);
    pub const P11: PlayerIdx<12> = PlayerIdx(11);
}

/// Defines indexes into a collection of type `PerPlayer<T, 13>`.
pub mod for13 {
    use super::PlayerIdx;
    pub const P0: PlayerIdx<13> = PlayerIdx(0);
    pub const P1: PlayerIdx<13> = PlayerIdx(1);
    pub const P2: PlayerIdx<13> = PlayerIdx(2);
    pub const P3: PlayerIdx<13> = PlayerIdx(3);
    pub const P4: PlayerIdx<13> = PlayerIdx(4);
    pub const P5: PlayerIdx<13> = PlayerIdx(5);
    pub const P6: PlayerIdx<13> = PlayerIdx(6);
    pub const P7: PlayerIdx<13> = PlayerIdx(7);
    pub const P8: PlayerIdx<13> = PlayerIdx(8);
    pub const P9: PlayerIdx<13> = PlayerIdx(9);
    pub const P10: PlayerIdx<13> = PlayerIdx(10);
    pub const P11: PlayerIdx<13> = PlayerIdx(11);
    pub const P12: PlayerIdx<13> = PlayerIdx(12);
}

/// Defines indexes into a collection of type `PerPlayer<T, 14>`.
pub mod for14 {
    use super::PlayerIdx;
    pub const P0: PlayerIdx<14> = PlayerIdx(0);
    pub const P1: PlayerIdx<14> = PlayerIdx(1);
    pub const P2: PlayerIdx<14> = PlayerIdx(2);
    pub const P3: PlayerIdx<14> = PlayerIdx(3);
    pub const P4: PlayerIdx<14> = PlayerIdx(4);
    pub const P5: PlayerIdx<14> = PlayerIdx(5);
    pub const P6: PlayerIdx<14> = PlayerIdx(6);
    pub const P7: PlayerIdx<14> = PlayerIdx(7);
    pub const P8: PlayerIdx<14> = PlayerIdx(8);
    pub const P9: PlayerIdx<14> = PlayerIdx(9);
    pub const P10: PlayerIdx<14> = PlayerIdx(10);
    pub const P11: PlayerIdx<14> = PlayerIdx(11);
    pub const P12: PlayerIdx<14> = PlayerIdx(12);
    pub const P13: PlayerIdx<14> = PlayerIdx(13);
}

/// Defines indexes into a collection of type `PerPlayer<T, 15>`.
pub mod for15 {
    use super::PlayerIdx;
    pub const P0: PlayerIdx<15> = PlayerIdx(0);
    pub const P1: PlayerIdx<15> = PlayerIdx(1);
    pub const P2: PlayerIdx<15> = PlayerIdx(2);
    pub const P3: PlayerIdx<15> = PlayerIdx(3);
    pub const P4: PlayerIdx<15> = PlayerIdx(4);
    pub const P5: PlayerIdx<15> = PlayerIdx(5);
    pub const P6: PlayerIdx<15> = PlayerIdx(6);
    pub const P7: PlayerIdx<15> = PlayerIdx(7);
    pub const P8: PlayerIdx<15> = PlayerIdx(8);
    pub const P9: PlayerIdx<15> = PlayerIdx(9);
    pub const P10: PlayerIdx<15> = PlayerIdx(10);
    pub const P11: PlayerIdx<15> = PlayerIdx(11);
    pub const P12: PlayerIdx<15> = PlayerIdx(12);
    pub const P13: PlayerIdx<15> = PlayerIdx(13);
    pub const P14: PlayerIdx<15> = PlayerIdx(14);
}

/// Defines indexes into a collection of type `PerPlayer<T, 16>`.
pub mod for16 {
    use super::PlayerIdx;
    pub const P0: PlayerIdx<16> = PlayerIdx(0);
    pub const P1: PlayerIdx<16> = PlayerIdx(1);
    pub const P2: PlayerIdx<16> = PlayerIdx(2);
    pub const P3: PlayerIdx<16> = PlayerIdx(3);
    pub const P4: PlayerIdx<16> = PlayerIdx(4);
    pub const P5: PlayerIdx<16> = PlayerIdx(5);
    pub const P6: PlayerIdx<16> = PlayerIdx(6);
    pub const P7: PlayerIdx<16> = PlayerIdx(7);
    pub const P8: PlayerIdx<16> = PlayerIdx(8);
    pub const P9: PlayerIdx<16> = PlayerIdx(9);
    pub const P10: PlayerIdx<16> = PlayerIdx(10);
    pub const P11: PlayerIdx<16> = PlayerIdx(11);
    pub const P12: PlayerIdx<16> = PlayerIdx(12);
    pub const P13: PlayerIdx<16> = PlayerIdx(13);
    pub const P14: PlayerIdx<16> = PlayerIdx(14);
    pub const P15: PlayerIdx<16> = PlayerIdx(15);
}
