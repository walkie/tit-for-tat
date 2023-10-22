use crate::{Game, History, Outcome, Payoff, PlayerIndex, Transcript};
use std::fmt::Debug;

/// The strategic context in which a player makes a move during a repeated game.
///
/// This type includes all information, besides the definition of the stage game, that a strategy
/// for a repeated game may use to compute its next move. It includes the history of past games
/// played, the game state of the current iteration, and (for sequential games) the transcript of
/// moves played so far in the current iteration.
pub struct Context<G: Game<P>, const P: usize> {
    current_player: Option<PlayerIndex<P>>,
    game_state: Option<G::State>,
    in_progress: Transcript<G::Move, P>,
    history: History<G::Kind, G::Move, G::Utility, P>,
}

impl<G: Game<P>, const P: usize> Context<G, P> {
    pub fn new(init_state: G::State) -> Self {
        Context {
            current_player: None,
            game_state: Some(init_state),
            in_progress: Transcript::new(),
            history: History::new(),
        }
    }

    pub fn set_current_player(&mut self, player: PlayerIndex<P>) {
        self.current_player = Some(player);
    }

    pub fn unset_current_player(&mut self) {
        self.current_player = None;
    }

    pub fn set_game_state(&mut self, state: G::State) {
        self.game_state = Some(state);
    }

    pub fn update_game_state<F>(&mut self, update: F)
    where
        F: FnOnce(G::State) -> Option<G::State>,
    {
        if let Some(state) = Option::take(&mut self.game_state) {
            self.game_state = update(state);
        }
    }

    pub fn complete(
        &mut self,
        outcome: Outcome<G::Kind, G::Move, G::Utility, P>,
    ) -> &Outcome<G::Kind, G::Move, G::Utility, P> {
        self.history.add(outcome)
    }

    pub fn current_player(&self) -> Option<PlayerIndex<P>> {
        self.current_player
    }

    pub fn game_state(&self) -> Option<&G::State> {
        self.game_state.as_ref()
    }

    pub fn in_progress(&self) -> &Transcript<G::Move, P> {
        &self.in_progress
    }

    pub fn history(&self) -> &History<G::Kind, G::Move, G::Utility, P> {
        &self.history
    }

    pub fn score(&self) -> Payoff<G::Utility, P> {
        self.history.score()
    }
}

// Unfortunately, we have to manually implement the following traits because the derived instances
// assume that the game type `G` must also satisfy the traits, which isn't necessary.

impl<G: Game<P>, const P: usize> Clone for Context<G, P> {
    fn clone(&self) -> Self {
        Context {
            current_player: self.current_player,
            game_state: self.game_state.clone(),
            in_progress: self.in_progress.clone(),
            history: self.history.clone(),
        }
    }
}

// Implement this manually because the derived implementation incorrectly thinks that `G` must
// implement `Debug`.
impl<G: Game<P>, const P: usize> Debug for Context<G, P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Context")
            .field("current_player", &self.current_player)
            .field("game_state", &self.game_state)
            .field("in_progress", &self.in_progress)
            .field("history", &self.history)
            .finish()
    }
}

impl<G: Game<P>, const P: usize> PartialEq for Context<G, P> {
    fn eq(&self, other: &Self) -> bool {
        self.current_player == other.current_player
            && self.game_state == other.game_state
            && self.in_progress == other.in_progress
            && self.history == other.history
    }
}
