//! This module defines the [`PerPlayer`] data type, which stores one element corresponding to each
//! player in a game.

use derive_more::{AsMut, AsRef, From, Into, IntoIterator};
use std::ops::{Index, IndexMut};

/// A struct that stores one element of type `T` corresponding to each player in a game. The number
/// of players in the game, `NUM_PLAYERS`, is encoded in the type of the struct.
#[derive(Clone, Debug, Eq, PartialEq, AsMut, AsRef, From, Into, IntoIterator)]
pub struct PerPlayer<T, const NUM_PLAYERS: usize> {
    data: [T; NUM_PLAYERS],
}

/// An index into a [`PerPlayer`] struct.
pub struct PlayerIdx<const NUM_PLAYERS: usize>(usize);

impl<const NUM_PLAYERS: usize> PlayerIdx<NUM_PLAYERS> {
    pub fn new(index: usize) -> Option<Self> {
        if index < NUM_PLAYERS {
            Some(PlayerIdx(index))
        } else {
            None
        }
    }
}

impl<T, const NUM_PLAYERS: usize> PerPlayer<T, NUM_PLAYERS> {
    /// Create a new [`PerPlayer`] struct from the given array.
    pub fn new(data: [T; NUM_PLAYERS]) -> Self {
        PerPlayer { data }
    }

    /// Get the number of players in the game, which corresponds to the number of elements in this
    /// struct.
    pub fn num_players(&self) -> usize {
        NUM_PLAYERS
    }

    pub fn for_player(&self, i: usize) -> Option<&T> {
        if i < NUM_PLAYERS {
            Some(&self.data[i])
        } else {
            None
        }
    }

    pub fn for_player_mut(&mut self, i: usize) -> Option<&mut T> {
        if i < NUM_PLAYERS {
            Some(&mut self.data[i])
        } else {
            None
        }
    }
}

impl<T, const NUM_PLAYERS: usize> Index<PlayerIdx<NUM_PLAYERS>> for PerPlayer<T, NUM_PLAYERS> {
    type Output = T;
    fn index(&self, idx: PlayerIdx<NUM_PLAYERS>) -> &T {
        unsafe { self.data.get_unchecked(idx.0) }
    }
}

impl<T, const NUM_PLAYERS: usize> IndexMut<PlayerIdx<NUM_PLAYERS>> for PerPlayer<T, NUM_PLAYERS> {
    fn index_mut(&mut self, idx: PlayerIdx<NUM_PLAYERS>) -> &mut T {
        unsafe { self.data.get_unchecked_mut(idx.0) }
    }
}

/// Defines a set of indexes into a struct of type `PerPlayer<T, 1>`.
pub mod for1 {
    use super::PlayerIdx;
    pub const P0: PlayerIdx<1> = PlayerIdx(0);
}

/// Defines a set of indexes into a struct of type `PerPlayer<T, 2>`.
pub mod for2 {
    use super::PlayerIdx;
    pub const P0: PlayerIdx<2> = PlayerIdx(0);
    pub const P1: PlayerIdx<2> = PlayerIdx(1);
}

/// Defines a set of indexes into a struct of type `PerPlayer<T, 3>`.
pub mod for3 {
    use super::PlayerIdx;
    pub const P0: PlayerIdx<3> = PlayerIdx(0);
    pub const P1: PlayerIdx<3> = PlayerIdx(1);
    pub const P2: PlayerIdx<3> = PlayerIdx(2);
}

/// Defines a set of indexes into a struct of type `PerPlayer<T, 4>`.
pub mod for4 {
    use super::PlayerIdx;
    pub const P0: PlayerIdx<4> = PlayerIdx(0);
    pub const P1: PlayerIdx<4> = PlayerIdx(1);
    pub const P2: PlayerIdx<4> = PlayerIdx(2);
    pub const P3: PlayerIdx<4> = PlayerIdx(3);
}

/// Defines a set of indexes into a struct of type `PerPlayer<T, 5>`.
pub mod for5 {
    use super::PlayerIdx;
    pub const P0: PlayerIdx<5> = PlayerIdx(0);
    pub const P1: PlayerIdx<5> = PlayerIdx(1);
    pub const P2: PlayerIdx<5> = PlayerIdx(2);
    pub const P3: PlayerIdx<5> = PlayerIdx(3);
    pub const P4: PlayerIdx<5> = PlayerIdx(4);
}

/// Defines a set of indexes into a struct of type `PerPlayer<T, 6>`.
pub mod for6 {
    use super::PlayerIdx;
    pub const P0: PlayerIdx<6> = PlayerIdx(0);
    pub const P1: PlayerIdx<6> = PlayerIdx(1);
    pub const P2: PlayerIdx<6> = PlayerIdx(2);
    pub const P3: PlayerIdx<6> = PlayerIdx(3);
    pub const P4: PlayerIdx<6> = PlayerIdx(4);
    pub const P5: PlayerIdx<6> = PlayerIdx(5);
}

/// Defines a set of indexes into a struct of type `PerPlayer<T, 7>`.
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

/// Defines a set of indexes into a struct of type `PerPlayer<T, 8>`.
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

/// Defines a set of indexes into a struct of type `PerPlayer<T, 9>`.
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

/// Defines a set of indexes into a struct of type `PerPlayer<T, 10>`.
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

/// Defines a set of indexes into a struct of type `PerPlayer<T, 11>`.
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

/// Defines a set of indexes into a struct of type `PerPlayer<T, 12>`.
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

/// Defines a set of indexes into a struct of type `PerPlayer<T, 13>`.
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

/// Defines a set of indexes into a struct of type `PerPlayer<T, 14>`.
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

/// Defines a set of indexes into a struct of type `PerPlayer<T, 15>`.
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

/// Defines a set of indexes into a struct of type `PerPlayer<T, 16>`.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn player_indexing() {
        let ps = PerPlayer::new(["a", "b", "c", "d"]);
        let p0 = PlayerIdx::new(0).unwrap();
        let p1 = PlayerIdx::new(1).unwrap();
        let p2 = PlayerIdx::new(2).unwrap();
        let p3 = PlayerIdx::new(3).unwrap();
        assert_eq!("a", ps[p0]);
        assert_eq!("b", ps[p1]);
        assert_eq!("c", ps[p2]);
        assert_eq!("d", ps[p3]);
        assert_eq!("b", ps[for4::P1]);

        let p5 = PlayerIdx::<4>::new(4);
        assert!(p5.is_none());
    }
}
