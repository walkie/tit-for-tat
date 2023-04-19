pub(crate) mod distribution;
pub(crate) mod dominated;
pub(crate) mod history;
pub(crate) mod moves;
pub(crate) mod normal;
pub(crate) mod outcome;
pub(crate) mod payoff;
pub(crate) mod per_player;
pub(crate) mod play;
pub(crate) mod player;
pub(crate) mod profile;
pub(crate) mod record;
pub(crate) mod simultaneous;
pub(crate) mod strategy;
pub(crate) mod transcript;
pub(crate) mod tree;

pub use distribution::*;
pub use dominated::*;
pub use history::*;
pub use moves::*;
pub use normal::*;
pub use outcome::*;
pub use payoff::*;
pub use per_player::*;
pub use play::*;
pub use player::*;
pub use profile::*;
pub use record::*;
pub use simultaneous::*;
pub use strategy::*;
pub use transcript::*;
pub use tree::*;

// /// A prelude that includes all of the definitions used in defining and executing
// /// [extensive-form games](crate::seq::Extensive).
// pub mod extensive {
//
// }

/// A prelude that includes all of the definitions used in defining and executing (possibly
/// repeated) [normal-form games](crate::Normal).
pub mod norm {
    pub use crate::distribution::*;
    pub use crate::dominated::*;
    pub use crate::moves::*;
    pub use crate::normal::*;
    pub use crate::outcome::*;
    pub use crate::payoff::*;
    pub use crate::per_player::*;
    pub use crate::play::*;
    pub use crate::profile::*;
    pub use crate::strategy::*;
}

/// A prelude that includes all of the definitions used in defining and executing (non-finite,
/// possibly repeated) [simultaneous-move games](crate::Simultaneous).
pub mod sim {
    pub use crate::distribution::*;
    pub use crate::moves::IsMove;
    pub use crate::outcome::Outcome;
    pub use crate::payoff::*;
    pub use crate::per_player::*;
    pub use crate::play::*;
    pub use crate::profile::*;
    pub use crate::simultaneous::*;
    pub use crate::strategy::*;
}
