use std::fmt;
use std::rc::Rc;

use crate::{Action, Game, History, PlayerIndex, Turn};

pub struct Repeated<G: Game<P>, const P: usize> {
    stage_game: Rc<G>,
    repetitions: usize,
}

pub struct RepeatedState<G: Game<P>, const P: usize> {
    stage_game: Rc<G>,
    stage_state: Rc<G::State>,
    completed: History<G, P>,
    remaining: usize,
}

impl<G: Game<P> + 'static, const P: usize> Repeated<G, P> {
    pub fn new(stage_game: Rc<G>, repetitions: usize) -> Self {
        Repeated {
            stage_game,
            repetitions,
        }
    }

    pub fn stage_game(&self) -> &Rc<G> {
        &self.stage_game
    }

    pub fn repetitions(&self) -> usize {
        self.repetitions
    }
}

impl<G: Game<P>, const P: usize> RepeatedState<G, P> {
    pub fn new(stage_game: Rc<G>, remaining: usize) -> Self {
        let stage_state = stage_game.rules().state.clone();
        RepeatedState {
            stage_game,
            stage_state,
            completed: History::empty(),
            remaining,
        }
    }

    pub fn state_view(&self, player: PlayerIndex<P>) -> G::View {
        self.stage_game
            .state_view(self.stage_state.as_ref(), player)
    }

    pub fn history(&self) -> &History<G, P> {
        &self.completed
    }

    pub fn remaining(&self) -> usize {
        self.remaining
    }
}

fn lift_turn<'g, G: Game<P> + 'g, const P: usize>(
    stage_game: &'g G,
    state: Rc<RepeatedState<G, P>>,
    turn: Turn<'g, G::State, G::Move, G::Outcome, P>,
) -> Turn<'g, RepeatedState<G, P>, G::Move, History<G, P>, P> {
    match turn.action {
        Action::Players {
            to_move: players,
            next,
        } => Turn::players(
            state.clone(),
            players,
            move |repeated_state: Rc<RepeatedState<G, P>>, moves: Vec<G::Move>| match next(
                repeated_state.stage_state.clone(),
                moves,
            ) {
                Ok(stage_turn) => {
                    let mut next_state = (*state).clone();
                    next_state.stage_state = stage_turn.state.clone();

                    Ok(lift_turn(stage_game, Rc::new(next_state), stage_turn))
                }

                Err(kind) => Err(kind),
            },
        ),

        Action::Chance { distribution, next } => Turn::chance(
            state.clone(),
            distribution,
            move |repeated_state: Rc<RepeatedState<G, P>>, the_move: G::Move| match next(
                repeated_state.stage_state.clone(),
                the_move,
            ) {
                Ok(stage_turn) => {
                    let mut next_state = (*state).clone();
                    next_state.stage_state = stage_turn.state.clone();

                    Ok(lift_turn(stage_game, Rc::new(next_state), stage_turn))
                }

                Err(kind) => Err(kind),
            },
        ),

        Action::End { outcome } if state.remaining > 0 => {
            let stage_turn = stage_game.rules();

            let mut next_state = (*state).clone();
            next_state.stage_state = stage_turn.state.clone();

            next_state.completed.add(outcome);
            next_state.remaining -= 1;

            lift_turn(stage_game, Rc::new(next_state), stage_turn)
        }

        Action::End { outcome } => {
            let mut history = state.completed.clone(); // TODO avoid this clone
            history.add(outcome);

            Turn::end(state, history)
        }
    }
}

impl<G: Game<P> + 'static, const P: usize> Game<P> for Repeated<G, P> {
    type Move = G::Move;
    type Utility = G::Utility;
    type Outcome = History<G, P>;
    type State = RepeatedState<G, P>;
    type View = RepeatedState<G, P>; // TODO add RepeatedStateView or some other solution

    fn rules(&self) -> Turn<RepeatedState<G, P>, G::Move, History<G, P>, P> {
        let init_state = Rc::new(RepeatedState::new(
            self.stage_game.clone(),
            self.repetitions - 1,
        ));

        lift_turn(
            self.stage_game.as_ref(),
            init_state,
            self.stage_game.rules(),
        )
    }

    fn state_view(
        &self,
        state: &RepeatedState<G, P>,
        _player: PlayerIndex<P>,
    ) -> RepeatedState<G, P> {
        state.clone() // TODO
    }

    fn is_valid_move(
        &self,
        state: &Self::State,
        player: PlayerIndex<P>,
        the_move: Self::Move,
    ) -> bool {
        self.stage_game
            .is_valid_move(&state.stage_state, player, the_move)
    }
}

impl<G: Game<P>, const P: usize> Clone for RepeatedState<G, P> {
    fn clone(&self) -> Self {
        RepeatedState {
            stage_game: self.stage_game.clone(),
            stage_state: self.stage_state.clone(),
            completed: self.completed.clone(),
            remaining: self.remaining,
        }
    }
}

impl<G: Game<P>, const P: usize> fmt::Debug for RepeatedState<G, P> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("RepeatedState")
            .field("stage_state", &self.stage_state)
            .field("completed", &self.completed)
            .field("remaining", &self.remaining)
            .finish()
    }
}

impl<G: Game<P>, const P: usize> PartialEq for RepeatedState<G, P> {
    fn eq(&self, other: &Self) -> bool {
        self.stage_state == other.stage_state
            && self.completed == other.completed
            && self.remaining == other.remaining
    }
}
