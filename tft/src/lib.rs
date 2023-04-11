pub(crate) mod distribution;
pub(crate) mod moves;
pub(crate) mod payoff;
pub(crate) mod per_player;
pub(crate) mod player;
pub(crate) mod strategy;

pub use distribution::*;
pub use moves::*;
pub use payoff::*;
pub use per_player::*;
pub use player::*;
pub use strategy::*;

/// Definitions specific to sequential games.
pub mod seq {
    pub(crate) mod outcome;
    pub(crate) mod transcript;
    pub(crate) mod tree;

    // pub(crate) mod extensive;

    pub use outcome::*;
    pub use transcript::*;
    pub use tree::*;
}

/// Definitions specific to simultaneous games.
pub mod sim {
    pub(crate) mod dominated;
    pub(crate) mod normal;
    pub(crate) mod outcome;
    pub(crate) mod profile;
    // pub(crate) mod repeated;
    pub(crate) mod simultaneous;

    pub use dominated::*;
    pub use normal::*;
    pub use outcome::*;
    pub use profile::*;
    // pub use repeated::*;
    pub use simultaneous::*;
}

// /// A prelude that includes all of the definitions used in defining and executing
// /// [extensive-form games](crate::Extensive).
// pub mod extensive {
//
// }

/// A prelude that includes all of the definitions used in defining and executing (possibly
/// repeated) [normal-form games](crate::Normal).
pub mod normal {
    pub use crate::distribution::*;
    pub use crate::moves::*;
    pub use crate::payoff::*;
    pub use crate::per_player::*;
    pub use crate::player::*;
    pub use crate::strategy::*;

    pub use crate::sim::dominated::*;
    pub use crate::sim::normal::*;
    pub use crate::sim::outcome::*;
    pub use crate::sim::profile::*;
}

/// A prelude that includes all of the definitions used in defining and executing (non-finite,
/// possibly repeated) [simultaneous-move games](crate::Simultaneous).
pub mod simultaneous {
    pub use crate::distribution::*;
    pub use crate::moves::IsMove;
    pub use crate::payoff::*;
    pub use crate::per_player::*;
    pub use crate::player::*;
    pub use crate::strategy::*;

    pub use crate::sim::outcome::Outcome;
    pub use crate::sim::profile::*;
    pub use crate::sim::simultaneous::*;
}
