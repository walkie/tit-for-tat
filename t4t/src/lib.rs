#![warn(missing_docs)]

//! Tit-for-tat (t4t) is a [game theory][wiki-game-theory] library focusing on expressiveness, type
//! safety, and experimentation. It provides flexible types and traits for defining games and
//! strategies, then executing them to observe the results.
//!
//! The companion crate [t4t-games][games-crate] provides some example games and strategies.
//!
//! # Features
//!
//! **Defining games**
//!
//! The library provides a variety of types for defining common kinds of games:
//!
//! - [`GameTree`]: A very generic game-tree representation. This is not very convenient to use
//!   directly, but all games are eventually translated into this type in order to be executed.
//!
//! - [`Normal`]: A general representation of n-ary [normal-form games][normal-form-game].
//!   An arbitrary number of players simultaneously play a single move selected from a finite set
//!   of available moves.
//!
//! - [`Simultaneous`]: N-ary [simultaneous games][simultaneous-game]. Similar to [`Normal`],
//!   except the set of moves available to each player may be non-finite.
//!
//! - `Extensive` (coming soon): A simple representation of [extensive-form games][extensive-form-game],
//!   that is, games represented as complete game trees, where players take turns making moves,
//!   possibly with moves of chance interspersed.
//!
//! - `StateBased` (coming soon): Games that revolve around manipulating a shared state.
//!
//! - [`Repeated`]: Games where another game is played repeatedly a given number of times.
//!
//! Each of these game types represents a class of games that work in a similar way. Most new games
//! can can be defined using the constructors on these types.
//!
//! Each game type implements two or three traits:
//!
//! - [`Game`]: Defines associated types used by the rest of the library, such as the type of moves
//!   played by players during the game, the type of the intermediate game state, and the type of
//!   utility values awarded to players at the end of the game.
//!
//! - [`Playable`]: Defines a common interface for playing games via translation to the [`GameTree`]
//!   type.
//!
//! - [`FiniteGame`]: Provides a method to enumerate the moves available to a player in games where
//!   this set is finite.
//!
//! If you'd like to define your own completely new game forms or transformations, you will need to
//! implement these traits. The best way to learn how to do this is currently to look at the source
//! code for this crate and the [t4t-games][games-crate] crate.
//!
//! **Defining players and strategies**
//!
//! A [`Player`] consists of an identifying name and a [`Strategy`].
//!
//! A [`Strategy`] provides a method to get the next move to play in a game, given the current
//! [strategic context](crate::Context). For example, for repeated games, the strategic context
//! includes the history of games played so far.
//!
//! There are several associated functions on the [`Strategy`] type for defining common types of
//! strategies.
//!
//! **Running tournaments**
//!
//! The [`Tournament`] type provides a convenient way to efficiently play many instances of a game
//! in parallel, with all combinations or permutations of a set of players, aggregating the results.
//!
//! # Example
//!
//! The following example illustrates defining a few simple games and strategies, then executing
//! them.
//!
//! ```
//! use itertools::*;
//! use std::sync::Arc;
//! use t4t::*;
//!
//! // Possibles moves in social dilemma games, like the Prisoner's Dilemma.
//! #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
//! enum DilemmaMove { Cooperate, Defect }
//!
//! // The type of a 2-player social dilemma game with integer payoffs.
//! type Dilemma = Normal<DilemmaMove, i32, 2>;
//!
//! // Define the Prisoner's Dilemma.
//! let pd: Dilemma = Normal::symmetric(
//!     vec![DilemmaMove::Cooperate, DilemmaMove::Defect],
//!     vec![2, 0, 3, 1]
//! ).unwrap();
//!
//! // Define two functions that produce players for playing social dilemma games. The game type is
//! // generic so that we can reuse these players later for repeated prisoner's dilemma.
//! fn nice<G: Game<2, Move = DilemmaMove>>() -> Player<G, 2> {
//!     Player::new("Nice".to_string(), || Strategy::pure(DilemmaMove::Cooperate))
//! }
//!
//! fn mean<G: Game<2, Move = DilemmaMove>>() -> Player<G, 2> {
//!     Player::new("Mean".to_string(), || Strategy::pure(DilemmaMove::Defect))
//! }
//!
//! // Play the game!
//! let result = pd.play(&Matchup::from_players([nice(), mean()]));
//! assert_eq!(result.unwrap().payoff(), &Payoff::from([0, 3]));
//!
//! // Define the repeated prisoner's dilemma.
//! let rpd: Repeated<Dilemma, 2> = Repeated::new(Arc::new(pd), 100);
//!
//! // Define a player that plays the famous tit-for-tat strategy.
//! let tit_for_tat: Player<Repeated<Dilemma, 2>, 2> = Player::new(
//!     "Tit-for-Tat".to_string(),
//!     || Strategy::new(|context: &Context<RepeatedState<Dilemma, 2>, 2>| {
//!         context
//!             .state_view() // get the repeated game state
//!             .history() // get the completed game history from the state
//!             .moves_for_player(context.their_index()) // get the moves for the other player
//!             .last() // take their last move only
//!             .unwrap_or(DilemmaMove::Cooperate) // play that, or cooperate if it's the first move
//!     }),
//! );
//!
//! // Create a tournament in which every player plays every player, including themselves.
//! let tournament = Tournament::combinations_with_replacement(
//!     Arc::new(rpd),
//!     &vec![Arc::new(nice()), Arc::new(mean()), Arc::new(tit_for_tat)],
//! );
//!
//! // Run all the matches in parallel and accumulate the resulting scores.
//! let results = tournament.play();
//! assert_eq!(
//!     results.score().best_to_worst(),
//!     vec![
//!         ("Tit-for-Tat", 699),
//!         ("Mean", 602),
//!         ("Nice", 600),
//!     ],
//! );
//! ```
//!
//! # Design priorities
//!
//! **Expressiveness over performance**
//!
//! This library prioritizes expressiveness over performance. It aims to provide a powerful set of
//! abstractions for defining arbitrary games and strategies, without sacrificing type safety.
//!
//! This tradeoff is evident in the representation of [normal-form games](crate::Normal), which are
//! represented not as, say, a matrix of float-tuples, but instead as a function from generic
//! [move profiles](crate::Profile) to generic [payoffs](crate::Payoff). This enables normal-form
//! games of arbitrary size, between arbitrary numbers of players, and with arbitrary move and
//! utility values, but is somewhat less efficient than a simple matrix.
//!
//! Games are defined by translation into a generic [game tree](crate::GameTree) representation.
//!
//! A subtler but more extreme example of this tradeoff is how games are executed. The [`Game`] and
//! [`Playable`] traits are quite generic. Implementers of these traits do not implement the
//! execution of their game directly, but rather produce a [*description*](crate::GameTree) of how
//! the game is executed. This is less efficient, but enables generically transforming the execution
//! of a game, for example, defining new games that modify the behavior of existing games.
//!
//! An example of this capability in action is the [`Repeated`] type, which transforms any game into
//! a repeated game, modifying the original game's state and execution to enable players of the game
//! to see the history of games played so far.
//!
//! Of course, all things being equal, I'd still like things to run as fast as possible! However,
//! if your application deals only with 2x2 numeric, normal-form games, and you need to run
//! billions of iterations, this might not be the library you're looking for.
//!
//! **Experimentation over formal analysis**
//!
//! The library emphasizes exploring strategic situations through *executing* games and strategies
//! and observing the results, rather than through formal, mathematical analysis of games. This is
//! consistent with the expressiveness goal above, since many games that can be defined with the
//! library may not be amenable to formal analysis.
//!
//! However, the library will aim to provide analytic solutions where possible, since often a goal
//! of experimental game theory is to compare various analytic solutions with each other and with
//! other strategies.
//!
//! # This is a work in progress!
//!
//! The library is a work-in-progress and will continue expanding and evolving. For now, expect
//! breaking changes on even patch-level version changes. Minor version changes will indicate
//! significant new features.
//!
//! The type- and method-level documentation is very good in places, minimal in others.
//!
//! [Normal-form games](crate::Normal) are in good shape, and [repeated games](crate::Repeated) are
//! solid for perfect-information games. You can define [players](crate::Player) and
//! [strategies](crate::Strategy) for these games, and they can be played.
//!
//! There is a mechanism to efficiently run [tournaments](crate::Tournament) by playing a game with
//! all combinations or permutations of a set of entrants.
//!
//! There is a lot of infrastructure in place for sequential and state-based games, but the library
//! is still missing the main top-level types to make this convenient to use.
//!
//! Long-term, I'd like to add ways to visualize game executions and build games and strategies
//! interactively, but we'll see!
//!
//! [wiki-game-theory]: https://en.wikipedia.org/wiki/Game_theory
//! [normal-form-game]: https://en.wikipedia.org/wiki/Normal-form_game
//! [simultaneous-game]: https://en.wikipedia.org/wiki/Simultaneous_game
//! [extensive-form-game]: https://en.wikipedia.org/wiki/Extensive-form_game
//! [repeated-game]: https://en.wikipedia.org/wiki/Repeated_game
//! [games-crate]: https://crates.io/crates/t4t-games

pub(crate) mod distribution;
pub(crate) mod dominated;
pub(crate) mod error;
pub(crate) mod finite;
// pub(crate) mod extensive;
pub(crate) mod game;
pub(crate) mod history;
pub(crate) mod matchup;
pub(crate) mod moves;
pub(crate) mod normal;
pub(crate) mod outcome;
pub(crate) mod past;
pub(crate) mod payoff;
pub(crate) mod per_player;
pub(crate) mod playable;
pub(crate) mod player;
pub(crate) mod ply;
pub(crate) mod possible_profiles;
pub(crate) mod profile;
pub(crate) mod record;
pub(crate) mod repeated;
pub(crate) mod score;
pub(crate) mod simultaneous;
// pub(crate) mod state_based;
pub(crate) mod strategy;
pub(crate) mod summary;
pub(crate) mod tournament;
pub(crate) mod transcript;
pub(crate) mod tree;

pub use distribution::*;
pub use dominated::*;
pub use error::*;
pub use finite::*;
// pub use extensive::*;
pub use game::*;
pub use history::*;
pub use matchup::*;
pub use moves::*;
pub use normal::*;
pub use outcome::*;
pub use past::*;
pub use payoff::*;
pub use per_player::*;
pub use playable::*;
pub use player::*;
pub use ply::*;
pub use possible_profiles::*;
pub use profile::*;
pub use record::*;
pub use repeated::*;
pub use score::*;
pub use simultaneous::*;
// pub use state_based::*;
pub use strategy::*;
pub use summary::*;
pub use tournament::*;
pub use transcript::*;
pub use tree::*;
