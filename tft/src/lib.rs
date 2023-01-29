pub(crate) mod dominated;
pub(crate) mod moves;
pub(crate) mod normal;
pub(crate) mod outcome;
pub(crate) mod payoff;
pub(crate) mod per_player;
pub(crate) mod profile;
pub(crate) mod simultaneous;
// pub(crate) mod ext;
// pub(crate) mod seq;

pub use dominated::*;
pub use moves::*;
pub use normal::*;
pub use outcome::*;
pub use payoff::*;
pub use per_player::*;
pub use profile::*;
pub use simultaneous::*;
// pub use ext::*;
// pub use seq::*;

/// A prelude that includes all of the definitions used in defining and executing
/// [normal-form games](crate::Normal).
pub mod norm {
    pub use crate::dominated::*;
    pub use crate::moves::*;
    pub use crate::normal::*;
    pub use crate::outcome::*;
    pub use crate::payoff::*;
    pub use crate::per_player::*;
    pub use crate::profile::*;
}

/// A prelude that includes all of the definitions used in defining and executing
/// (non-finite) [simultaneous-move games](crate::Simultaneous).
pub mod sim {
    pub use crate::moves::IsMove;
    pub use crate::payoff::*;
    pub use crate::per_player::*;
    pub use crate::profile::*;
    pub use crate::simultaneous::*;
}
