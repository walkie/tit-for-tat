#![warn(missing_docs)]

//! This library provides example games and strategies implemented using the
//! [tit-for-tat (t4t)][t4t] library.
//!
//! The games are organized into modules, which define several related games and strategies for
//! playing them. The top-level documentation for each module provides a more detailed overview.
//!
//! # Dilemma games ([dilemma])
//!
//! This module includes a collection of 2x2 symmetric normal-form games, where each player may
//! cooperate or defect. It includes the classic [prisoner's dilemma][prisoner] game, along with
//! several related games such as [stag hunt][stag-hunt], [chicken][chicken], and more!
//!
//! The games in this module are typically played [repeated][repeated] several times, with the
//! payoffs accumulated.
//!
//! The module also includes [several well-known strategies][pd-strategies] for playing the
//! repeated forms of such games, including the famous [tit-for-tat strategy][tft-strategy] from
//! which the t4t library gets its name!
//!
//! # Rock-paper-scissors games ([rps])
//!
//! This module includes the classic [rock-paper-scissors][rps-game] game plus a few variant games
//! involving either more moves or more players.
//!
//! [t4t]: https://crates.io/crates/t4t
//! [prisoner]: https://en.wikipedia.org/wiki/Prisoner%27s_dilemma
//! [stag-hunt]: https://en.wikipedia.org/wiki/Stag_hunt
//! [chicken]: https://en.wikipedia.org/wiki/Chicken_(game)
//! [repeated]: https://en.wikipedia.org/wiki/Repeated_game
//! [pd-strategies]: http://www.prisoners-dilemma.com/common-strategy/
//! [tft-strategy]: https://en.wikipedia.org/wiki/Tit_for_tat
//! [rps-game]: https://en.wikipedia.org/wiki/Rock_paper_scissors

pub mod dilemma;
pub mod rps;
