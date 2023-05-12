pub(crate) mod distribution;
pub(crate) mod error;
pub(crate) mod history;
pub(crate) mod moves;
pub(crate) mod payoff;
pub(crate) mod per_player;
pub(crate) mod player;
pub(crate) mod strategy;

pub use distribution::*;
pub use error::*;
pub use history::*;
pub use moves::*;
pub use payoff::*;
pub use per_player::*;
pub use player::*;
pub use strategy::*;

/// Definitions specific to simultaneous games.
pub mod sim {
    pub(crate) mod context;
    pub(crate) mod dominated;
    pub(crate) mod game;
    pub(crate) mod normal;
    pub(crate) mod outcome;
    pub(crate) mod profile;
    pub(crate) mod simultaneous;

    pub use context::*;
    pub use dominated::*;
    pub use game::*;
    pub use normal::*;
    pub use outcome::*;
    pub use profile::*;
    pub use simultaneous::*;
}

/// Definitions specific to sequential games.
pub mod seq {
    pub(crate) mod context;
    // pub(crate) mod game;
    pub(crate) mod outcome;
    pub(crate) mod transcript;
    // pub(crate) mod tree;

    pub use context::*;
    // pub use game::*;
    pub use outcome::*;
    pub use transcript::*;
    // pub use tree::*;
}

/// Preludes for conveniently importing all definitions related to specific kinds of games.
pub mod prelude {

    /// All definitions needed to define and play [extensive-form games](crate::seq::Extensive).
    pub mod ext {}

    /// All definitions needed to define and play [normal-form games](crate::sim::Normal).
    pub mod norm {
        pub use crate::distribution::*;
        pub use crate::error::*;
        pub use crate::moves::*;
        pub use crate::payoff::*;
        pub use crate::per_player::*;
        pub use crate::strategy::*;

        pub use crate::sim::context::*;
        pub use crate::sim::dominated::*;
        pub use crate::sim::game::*;
        pub use crate::sim::normal::*;
        pub use crate::sim::outcome::*;
        pub use crate::sim::profile::*;
    }

    /// All definitions needed to define and play (non-finite)
    /// [simultaneous-move games](crate::sim::Simultaneous).
    pub mod sim {
        pub use crate::distribution::*;
        pub use crate::error::*;
        pub use crate::moves::Move;
        pub use crate::payoff::*;
        pub use crate::per_player::*;
        pub use crate::strategy::*;

        pub use crate::sim::context::*;
        pub use crate::sim::game::*;
        pub use crate::sim::outcome::Outcome;
        pub use crate::sim::profile::Profile;
        pub use crate::sim::simultaneous::*;
    }
}
