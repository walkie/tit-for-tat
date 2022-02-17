//! This module provides list utility functions and specialized list representations.

use derive_more::{From, Into};
use typed_index_collections::TiVec;

/// Identifies one player in the game.
#[derive(Clone, Copy, Debug, Eq, PartialEq, From, Into)]
pub struct PlayerId(usize);

/// Identifies one turn within a single game iteration.
#[derive(Clone, Copy, Debug, Eq, PartialEq, From, Into)]
pub struct TurnId(usize);

/// Identifies one iteration in an iterated game.
#[derive(Clone, Copy, Debug, Eq, PartialEq, From, Into)]
pub struct IterationId(usize);

/// A vector where each element corresponds to one player in the game.
pub type PerPlayer<T> = TiVec<PlayerId, T>;

/// A vector where each element corresponds to one turn within a single game iteration.
pub type PerTurn<T> = TiVec<TurnId, T>;

/// A vector where each element corresponds to one iteration of an iterated game.
pub type PerIteration<T> = TiVec<IterationId, T>;
