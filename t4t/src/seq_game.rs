use crate::seq::Outcome;
use crate::{Move, PerPlayer, Utility};

/// The strategic context in which a player makes a move during a repeated simultaneous game.
///
/// This type includes all information, besides the definition of the stage game, that a strategy
/// for a repeated game may use to compute its next move. It includes the history of past games
/// played, the game state of the current iteration, and (for sequential games) the transcript of
/// moves played so far in the current iteration.
pub type Context<M, U, const P: usize> = crate::Context<(), M, U, Outcome<M, U, P>, P>;

/// A player of a simultaneous game. Consists of a name and a [strategy](crate::Strategy).
///
/// A player's name should usually be unique with respect to all players playing the same game.
pub type Player<M, U, const P: usize> = crate::Player<Context<M, U, P>, M>;

/// A [per-player](crate::PerPlayer) collection of simultaneous game [players](Player).
pub type Players<M, U, const P: usize> = PerPlayer<Player<M, U, P>, P>;

/// The main interface for playing sequential games.
pub trait Game<const P: usize>: Sized {
    // TODO: Someday, when the associated_const_equality and/or generic_const_exprs features are
    // implemented, replace this trait's const generic P with the following associated constant.
    // const PLAYERS: usize;

    /// The type of moves played by players in this game.
    type Move: Move;

    /// The type of utility values awarded to each player at the end of the game.
    type Utility: Utility;

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

    // next_state, is_valid_move(player, state, move), etc.

    /// The number of players this game is for.
    fn num_players(&self) -> usize {
        P
    }

    // /// Play one iteration of the game and return the record of this game iteration, if successful.
    // ///
    // /// # Note to implementors
    // ///
    // /// In addition to returning the completed game record, this method should add the record to
    // /// the execution state using [crate::PlayState::add_record]. For sequential games, it will
    // /// also need to update the current game's transcript using [crate::PlayState::add_move] after
    // /// getting and executing each player's move.
    // fn play(
    //     &self,
    //     players: &mut Players<Self, P>,
    //     state: &mut PlayState<Self, P>,
    // ) -> PlayResult<GameRecord<Self, P>, Self, P>;

    // /// Play a game once with the given players, starting from the initial state.
    // fn play_once(&self, players: &mut Players<Self, P>) -> PlayResult<GameRecord<Self, P>, Self, P> {
    //     let mut state = PlayState::new(self);
    //     self.play(players, &mut state)
    // }

    // /// Play a given number of iterations of a game with the given players, starting from the
    // /// initial state. Returns the final execution state, if successful.
    // fn play_repeatedly(
    //     &self,
    //     players: &mut Players<Self, P>,
    //     iterations: u32,
    // ) -> PlayResult<PlayState<Self, P>, Self, P> {
    //     let mut state = PlayState::new(self);
    //     for _ in 0..iterations {
    //        self.play(players, &mut state)?;
    //     }
    //     Ok(state)
    // }
}