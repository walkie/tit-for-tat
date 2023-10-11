use crate::{Move, Record, Utility};

/// The main interface for playing sequential games.
pub trait Game<const P: usize>: Sized {
    // TODO: Someday, when the associated_const_equality and/or generic_const_exprs features are
    // implemented, replace this trait's const generic P with the following associated constant.
    // const PLAYERS: usize;

    /// The type of intermediate state used to support the execution of a single iteration of the
    /// game.
    ///
    /// For [simultaneous][crate::Simultaneous] and [normal-form][crate::Normal] games, this will
    /// be `()`, since no intermediate state is required. For [extensive-form] games, the state
    /// will be the location in the game tree. For state-based games, the state type will be
    /// whatever state is used to define the game.
    type State;

    /// The type of moves played by players in this game.
    type Move: Move;

    /// The type of utility values awarded to each player at the end of the game.
    type Utility: Utility;

    type Record: Record<Self::Utility, P>;

    /// The initial game state.
    fn initial_state(&self) -> Self::State;

    /// The number of players this game is for.
    fn num_players(&self) -> usize {
        P
    }
}
