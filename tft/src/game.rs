use num::Num;
use std::fmt::Debug;
use std::hash::Hash;

use crate::history::GameRecord;
use crate::payoff::Payoff;
use crate::per_player::PlayerIndex;
use crate::play::{PlayResult, PlayState};
use crate::player::Players;
use crate::profile::Profile;
use crate::transcript::Transcript;

/// A trait that collects the trait requirements of moves.
///
/// A blanket implementation covers all types that meet the requirements, so this trait should not
/// be implemented directly.
pub trait Move: Copy + Debug + Eq + Hash + Sized + 'static {}
impl<T: Copy + Debug + Eq + Hash + 'static> Move for T {}

/// A trait that collects the trait requirements of payoff utility values.
///
/// A blanket implementation covers all types that meet the requirements, so this trait should not
/// be implemented directly.
pub trait Utility: Copy + Debug + Default + Num + PartialOrd + Sized + 'static {}
impl<T: Copy + Debug + Default + Num + PartialOrd + 'static> Utility for T {}

/// The moves played during a single iteration of this game.
///
/// Although not enforced, this should probably be viewed as a "sealed" trait with two instances:
/// - [Profile][crate::Profile] for simultaneous games
/// - [Transcript][crate::Transcript] for sequential games
pub trait MoveRecord<M: Move, const P: usize>: Clone + Debug + Eq + Hash + Sized + 'static {}
impl<M: Move, const P: usize> MoveRecord<M, P> for Profile<M, P> {}
impl<M: Move, const P: usize> MoveRecord<M, P> for Transcript<M, P> {}

/// An interface for playing games.
pub trait Game<const P: usize>: Sized {
    // TODO: Someday, when the assocated_const_equality and/or generic_const_exprs features are
    // implemented, replace this trait's const generic P with the following associated constant.
    // const PLAYERS: usize;

    /// The type of moves played by players in this game.
    type Move: Move;

    /// The type of utility values awarded to each player at the end of the game.
    type Utility: Utility;

    /// The type used to record the moves played during a single iteration of this game. See the
    /// documentation at [crate::MoveRecord].
    type MoveRecord: MoveRecord<Self::Move, P>;

    /// The number of players this game is for.
    fn num_players(&self) -> usize {
        P
    }

    /// Play one iteration of the game and return the record of this game iteration, if successful.
    ///
    /// # Note to implementors
    ///
    /// In addition to returning the completed game record, this method should add the record to
    /// the execution state using [crate::PlayState::add_record]. For sequential games, it will
    /// also need to update the current game's transcript using [crate::PlayState::add_move] after
    /// getting and executing each player's move.
    fn play(
        &self,
        players: &mut Players<Self, P>,
        state: &mut PlayState<Self, P>,
    ) -> PlayResult<GameRecord<Self, P>, Self, P>;

    /// Play a game once with the given players, starting from the initial state.
    fn play_once(&self, players: &mut Players<Self, P>) -> PlayResult<GameRecord<Self, P>, Self, P> {
        let mut state = PlayState::new(self);
        self.play(players, &mut state)
    }

    /// Play a given number of iterations of a game with the given players, starting from the
    /// initial state. Returns the final execution state, if successful.
    fn play_repeatedly(
        &self,
        players: &mut Players<Self, P>,
        iterations: u32,
    ) -> PlayResult<PlayState<Self, P>, Self, P> {
        let mut state = PlayState::new(self);
        for _ in 0..iterations {
           self.play(players, &mut state)?;
        }
        Ok(state)
    }
}

pub trait Simultaneous<const P: usize>:
    Game<P, MoveRecord = Profile<<Self as Game<P>>::Move, P>> + Sized
{
    /// Get the payoff for the given strategy profile.
    ///
    /// This method may return an arbitrary payoff if given an
    /// [invalid profile](Simultaneous::is_valid_profile).
    fn payoff(&self, profile: Profile<Self::Move, P>) -> Payoff<Self::Utility, P>;

    /// Is this a valid move for the given player?
    fn is_valid_move_for_player(&self, player: PlayerIndex<P>, the_move: Self::Move);

    /// Is this a valid strategy profile? A profile is valid if each move is valid for the
    /// corresponding player.
    fn is_valid_profile(&self, profile: Profile<Self::Move, P>) -> bool;
}

pub trait Sequential<const P: usize>:
    Game<P, MoveRecord = Transcript<<Self as Game<P>>::Move, P>> + Sized
{
    /// The type of intermediate state used to support the execution of a single iteration of the
    /// game.
    ///
    /// For [simultaneous][crate::Simultaneous] and [normal-form][crate::Normal] games, this will
    /// be `()`, since no intermediate state is required. For [extensive-form] games, the state
    /// will be the location in the game tree. For state-based games, the state type will be
    /// whatever state is used to define the game.
    ///
    /// Note that this type is different from the similarly named [`PlayState`][crate::PlayState]
    /// type, which is used to support and track the results of repeated game execution.
    ///
    /// A `PlayState<G, P>` contains a value of type `G::State` as a component, representing the
    /// intermediate state of the current game iteration.
    type State;

    /// The initial game state.
    fn initial_state(&self) -> Self::State;


}
