use std::fmt::Debug;

use crate::{Action, Context, Exec, Move, Payoff, PlayerIndex, Players, Profile, Turn, Utility};

pub trait State: Clone + Debug + PartialEq {}
impl<T: Clone + Debug + PartialEq> State for T {}

/// A root trait that all games implement, mostly used for its associated types.
pub trait Game<const P: usize>: Sized {
    /// The type of moves played by players in this game.
    type Move: Move;

    /// The type of utility values awarded to each player at the end of the game.
    type Utility: Utility;

    type State: State;

    type View: State;

    /// The first turn of the game.
    fn rules(&self) -> Turn<Self, P>;

    fn state_view(&self, state: &Self::State, player: PlayerIndex<P>) -> Self::View;

    /// Is this a valid move in the given context?
    fn is_valid_move(
        &self,
        state: &Self::State,
        player: PlayerIndex<P>,
        the_move: Self::Move,
    ) -> bool;

    /// The number of players this game is for.
    fn num_players(&self) -> usize {
        P
    }

    fn play(&self, players: Players<Self, P>) {
        let mut next_turn = self.rules();

        loop {
            let state = next_turn.state;
            let action = next_turn.action;

            match &action {
                Action::Players { players, next } => {
                    let moves = players
                        .map(|player| {
                            let context = Context::default(); // TODO
                            player.next_move(&context)
                        })
                        .collect();
                    next_turn = next(state, moves);
                }

                Action::Chance { distribution, next } => {
                    let the_move = distribution.sample();
                    next_turn = next(state, the_move);
                }

                Action::Payoff { payoff, next } => match next {
                    Some(next) => {
                        next_turn = next(state);
                    }
                    None => {
                        break;
                    }
                },
            }
        }
    }
}

struct Repeated<G: Game<P>, const P: usize> {
    stage_game: G,
    repetitions: usize,
}

struct Remaining<S> {
    state: S,
    remaining: usize,
}

impl<S> Remaining<S> {
    pub fn new(state: S, remaining: usize) -> Self {
        Remaining { state, remaining }
    }

    pub fn stage_game_state(&self) -> &S {
        &self.state
    }

    pub fn remaining_repetitions(&self) -> usize {
        self.remaining
    }
}

impl<G: Game<P>, const P: usize> Repeated<G, P> {
    pub fn new(stage_game: G, repetitions: usize) -> Self {
        Repeated {
            stage_game,
            repetitions,
        }
    }

    fn lift_turn(&self, remaining: usize, turn: Turn<G, P>) -> Turn<Repeated<G, P>, P> {
        match turn.action {
            Action::Players { players, next } => Turn::new(
                Remaining::new(turn.state, remaining),
                Action::players(players, move |wrapped_state, moves| {
                    self.lift_turn(remaining, next(wrapped_state.state, moves))
                }),
            ),

            Action::Chance { distribution, next } => Turn::new(
                Remaining::new(turn.state, remaining),
                Action::chance(distribution, move |wrapped_state, the_move| {
                    self.lift_turn(remaining, next(wrapped_state.state, the_move))
                }),
            ),

            Action::Payoff { payoff, next } => match next {
                Some(next) => Turn::new(
                    Remaining::new(turn.state, remaining),
                    Action::intermediate_payoff(payoff, move |wrapped_state| {
                        self.lift_turn(remaining, next(wrapped_state.state))
                    }),
                ),

                None if remaining <= 0 => Turn::new(
                    Remaining::new(turn.state, 0),
                    Action::terminal_payoff(payoff),
                ),

                None => Turn::new(
                    Remaining::new(turn.state, remaining - 1),
                    Action::intermediate_payoff(payoff, |_| {
                        self.lift_turn(remaining, self.stage_game.rules())
                    }),
                ),
            },
        }
    }
}

impl<G: Game<P>, const P: usize> Game<P> for Repeated<G, P> {
    type Move = G::Move;
    type Utility = G::Utility;
    type State = Remaining<G::State>;
    type View = Remaining<G::View>;

    fn rules(&self) -> Turn<Self, P> {
        self.lift_turn(self.repetitions - 1, self.stage_game.rules())
    }

    fn state_view(&self, state: &Self::State, player: PlayerIndex<P>) -> Self::View {
        Remaining::new(self.state_view(&state.state, player), state.remaining)
    }

    fn is_valid_move(
        &self,
        state: &Self::State,
        player: PlayerIndex<P>,
        the_move: Self::Move,
    ) -> bool {
        self.stage_game
            .is_valid_move(&state.state, player, the_move)
    }
}
