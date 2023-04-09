pub(crate) mod distribution;
pub(crate) mod dominated;
pub(crate) mod iterated;
pub(crate) mod moves;
pub(crate) mod normal;
pub(crate) mod outcome;
pub(crate) mod payoff;
pub(crate) mod per_player;
pub(crate) mod play;
pub(crate) mod player;
pub(crate) mod profile;
pub(crate) mod simultaneous;
pub(crate) mod strategy;
pub(crate) mod transcript;
pub(crate) mod tree;

// pub(crate) mod extensive;
// pub(crate) mod seq;

pub use distribution::*;
pub use dominated::*;
pub use iterated::*;
pub use moves::*;
pub use normal::*;
pub use outcome::*;
pub use payoff::*;
pub use per_player::*;
pub use play::*;
pub use player::*;
pub use profile::*;
pub use simultaneous::*;
pub use strategy::*;
pub use transcript::*;
pub use tree::*;

// pub use extensive::*;
// pub use seq::*;

/// A prelude that includes all of the definitions used in defining and executing
/// [extensive-form games](crate::Extensive).
pub mod ext {
    pub use crate::transcript::*;
}

/// A prelude that includes all of the definitions used in defining and executing (possibly
/// iterated) [normal-form games](crate::Normal).
pub mod norm {
    pub use crate::distribution::*;
    pub use crate::dominated::*;
    pub use crate::iterated::*;
    pub use crate::moves::*;
    pub use crate::normal::*;
    pub use crate::outcome::*;
    pub use crate::payoff::*;
    pub use crate::per_player::*;
    pub use crate::play::*;
    pub use crate::player::*;
    pub use crate::profile::*;
    pub use crate::strategy::*;
}

/// A prelude that includes all of the definitions used in defining and executing (non-finite,
/// possibly iterated) [simultaneous-move games](crate::Simultaneous).
pub mod sim {
    pub use crate::distribution::*;
    pub use crate::iterated::*;
    pub use crate::moves::IsMove;
    pub use crate::outcome::Outcome;
    pub use crate::payoff::*;
    pub use crate::per_player::*;
    pub use crate::play::*;
    pub use crate::player::*;
    pub use crate::profile::*;
    pub use crate::simultaneous::*;
    pub use crate::strategy::*;
}
