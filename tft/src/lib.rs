pub mod core;
pub mod game;
pub mod solution;

/// Exports the most frequently used types and traits.
pub mod prelude {
    pub use crate::core::*;
    pub use crate::game::traits::*;
}
