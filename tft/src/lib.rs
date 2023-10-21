mod internal {
    pub trait Sealed {}
}

pub(crate) mod context;
pub(crate) mod distribution;
pub(crate) mod dominated;
pub(crate) mod error;
pub(crate) mod form;
pub(crate) mod game;
pub(crate) mod history;
pub(crate) mod moves;
pub(crate) mod normal;
pub(crate) mod outcome;
pub(crate) mod payoff;
pub(crate) mod per_player;
pub(crate) mod player;
pub(crate) mod profile;
pub(crate) mod strategic;
pub(crate) mod strategy;
pub(crate) mod transcript;
// pub(crate) mod tree;

pub use context::*;
pub use distribution::*;
pub use dominated::*;
pub use error::*;
pub use form::*;
pub use game::*;
pub use history::*;
pub use moves::*;
pub use normal::*;
pub use outcome::*;
pub use payoff::*;
pub use per_player::*;
pub use player::*;
pub use profile::*;
pub use strategic::*;
pub use strategy::*;
pub use transcript::*;
// pub use tree::*;

/// All definitions needed to define and play [extensive-form games](crate::seq::Extensive).
pub mod ext {}

/// All definitions needed to define and play [normal-form games](crate::sim::Normal).
pub mod norm {
    pub use crate::distribution::*;
    pub use crate::dominated::*;
    pub use crate::error::*;
    pub use crate::game::*;
    pub use crate::moves::*;
    pub use crate::normal::*;
    pub use crate::outcome::*;
    pub use crate::payoff::*;
    pub use crate::per_player::*;
    pub use crate::player::*;
    pub use crate::profile::*;
    pub use crate::strategy::*;
}

/// All definitions needed to define and play (non-finite)
/// [simultaneous-move games](crate::sim::Strategic).
pub mod sim {
    pub use crate::distribution::*;
    pub use crate::error::*;
    pub use crate::game::*;
    pub use crate::moves::Move;
    pub use crate::outcome::Outcome;
    pub use crate::payoff::*;
    pub use crate::per_player::*;
    pub use crate::profile::Profile;
    pub use crate::strategic::*;
    pub use crate::strategy::*;
}
