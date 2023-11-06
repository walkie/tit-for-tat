use crate::{Action, Game, History, PlayerIndex, Turn};
use std::rc::Rc;

struct Repeated<G: Game<P>, const P: usize> {
    stage_game: Rc<G>,
    repetitions: usize,
}

struct RepeatedState<G: Game<P>, const P: usize> {
    stage_game: Rc<G>,
    stage_state: Rc<G::State>,
    completed: History<G::Outcome, G::Utility, P>,
    remaining: usize,
}

impl<G: Game<P>, const P: usize> RepeatedState<G, P> {
    pub fn new(stage_game: Rc<G>, remaining: usize) -> Self {
        RepeatedState {
            stage_game,
            stage_state: Rc::new(stage_game.rules().state),
            completed: History::new(),
            remaining,
        }
    }

    // pub fn state_view(&self) -> &G::State {
    //     &self.stage_state
    // }

    pub fn remaining(&self) -> usize {
        self.remaining
    }
}

impl<G: Game<P>, const P: usize> Repeated<G, P> {
    pub fn new(stage_game: G, repetitions: usize) -> Self {
        Repeated {
            stage_game: Rc::new(stage_game),
            repetitions,
        }
    }

    fn lift_turn(
        &self,
        state: Rc<RepeatedState<G, P>>,
        turn: Turn<G, P>,
    ) -> Turn<Repeated<G, P>, P> {
        match turn.action {
            Action::Players { players, next } => {
                Turn::players(state, players, move |repeated_state, moves| {
                    let stage_turn = next(repeated_state.stage_state.clone(), moves);
                    let mut next_state = (*state).clone();
                    next_state.stage_state = stage_turn.state.clone();
                    self.lift_turn(Rc::new(next_state), stage_turn)
                })
            }

            Action::Chance { distribution, next } => {
                Turn::chance(state, distribution, move |repeated_state, the_move| {
                    let stage_turn = next(repeated_state.stage_state.clone(), the_move);

                    let mut next_state = (*state).clone();
                    next_state.stage_state = stage_turn.state.clone();

                    self.lift_turn(Rc::new(next_state), stage_turn)
                })
            }

            Action::Payoff { payoff, outcome } if state.remaining > 0 => {
                let stage_turn = self.stage_game.rules();

                let mut next_state = (*state).clone();
                next_state.stage_state = stage_turn.state.clone();

                let outcome = outcome(turn.state, payoff);
                next_state.completed.push(outcome);
                next_state.remaining -= 1;

                self.lift_turn(Rc::new(next_state), stage_turn)
            }

            Action::Payoff { payoff, outcome } => {
                let history = state.completed.clone(); // TODO avoid this clone

                let outcome = outcome(turn.state, payoff);
                history.push(outcome);

                Turn::payoff(state, history.score(), move |_, _| history)
            }
        }
    }
}

impl<G: Game<P>, const P: usize> Game<P> for Repeated<G, P> {
    type Move = G::Move;
    type Utility = G::Utility;
    type Outcome = History<G::Outcome, G::Utility, P>;
    type State = RepeatedState<G, P>;
    type View = RepeatedState<G, P>; // TODO add RepeatedStateView or some other solution

    fn rules(&self) -> Turn<Self, P> {
        let init_state = Rc::new(RepeatedState::new(
            self.stage_game.clone(),
            self.repetitions - 1,
        ));
        self.lift_turn(init_state, self.stage_game.rules())
    }

    fn state_view(&self, state: &Self::State, _player: PlayerIndex<P>) -> Self::View {
        (*state).clone()
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
