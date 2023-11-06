use crate::{
    Context, Game, Outcome, PerPlayer, PlayError, PlayErrorInContext, PlayResult, PlayerIndex,
    Sequential, Simultaneous, Strategy,
};

/// A [per-player](PerPlayer) collection of [players](Player), ready to play a game.
pub type Players<G, const P: usize> = PerPlayer<Player<G, P>, P>;

/// A player consists of a name and a [strategy](Strategy).
///
/// A player's name should usually be unique with respect to all players playing the same game.
pub struct Player<G: Game<P>, const P: usize> {
    name: String,
    strategy: Box<dyn Strategy<G, P>>,
}

impl<G: Game<P>, const P: usize> Player<G, P> {
    /// Construct a new player with the given name and strategy.
    pub fn new(name: String, strategy: impl Strategy<G, P> + 'static) -> Self {
        Player {
            name,
            strategy: Box::new(strategy),
        }
    }

    /// The player's name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the player's next move to play given a particular game state.
    pub fn next_move(&mut self, context: &Context<G, P>) -> G::Move {
        self.strategy.next_move(context)
    }
}

pub trait Playable<K: Kind, const P: usize>: Game<P, Kind = K> {
    /// Play an iteration of the game to completion in the given context. Update the context and
    /// return the outcome if successful.
    #[allow(clippy::type_complexity)]
    fn play_in_context<'c>(
        &self,
        players: &mut Players<Self, P>,
        context: &'c mut Context<Self, P>,
    ) -> PlayResult<&'c Outcome<K, Self::Move, Self::Utility, P>, Self, P>;

    /// Play a game once with the given players, returning the outcome if successful.
    #[allow(clippy::type_complexity)]
    fn play_once(
        &self,
        players: &mut Players<Self, P>,
    ) -> PlayResult<Outcome<K, Self::Move, Self::Utility, P>, Self, P> {
        let mut context = Context::new(Self::initial_state(self));
        let outcome = self.play_in_context(players, &mut context)?;
        Ok(outcome.clone())
    }

    /// Play a given number of iterations of a game with the given players, starting from the
    /// initial state. Return the final context if successful.
    fn play_repeatedly(
        &self,
        iterations: u32,
        players: &mut Players<Self, P>,
    ) -> PlayResult<Context<Self, P>, Self, P> {
        let mut context = Context::new(Self::initial_state(self));
        for _ in 0..iterations {
            self.play_in_context(players, &mut context)?;
        }
        Ok(context)
    }
}

impl<G: Simultaneous<P>, const P: usize> Playable<Sim, P> for G {
    fn play_in_context<'c>(
        &self,
        players: &mut Players<Self, P>,
        context: &'c mut Context<Self, P>,
    ) -> PlayResult<&'c Outcome<Sim, Self::Move, Self::Utility, P>, Self, P> {
        // get a move from each player
        let profile = PerPlayer::generate(|i| {
            context.set_current_player(i);
            players[i].next_move(context)
        });

        // check that all players played valid moves
        for i in PlayerIndex::all_indexes() {
            context.set_current_player(i);
            if !self.is_valid_move_in_context(context, profile[i]) {
                return Err(PlayErrorInContext::new(
                    context.clone(),
                    PlayError::InvalidMove(i, profile[i]),
                ));
            }
        }
        context.unset_current_player();

        // compute the payoff, update the game state, and return the outcome
        let payoff = self.payoff_in_context(profile, context);
        Ok(context.complete(Outcome::new(profile, payoff)))
    }
}

impl<G: Sequential<P>, const P: usize> Playable<Seq, P> for G {
    fn play_in_context<'c>(
        &self,
        players: &mut Players<Self, P>,
        context: &'c mut Context<Self, P>,
    ) -> PlayResult<&'c Outcome<Seq, Self::Move, Self::Utility, P>, Self, P> {
        let mut state = self.initial_state();
        let mut action = self.initial_action();
        loop {
            match &action {
                Action::Turns {
                    players: to_move,
                    next,
                } => {
                    let mut moves = Vec::with_capacity(to_move.len());
                    for &player in to_move {
                        context.set_current_player(player);
                        let the_move = players[player].next_move(context);
                        if !self.is_valid_move_in_context(context, the_move) {
                            return Err(PlayErrorInContext::new(
                                context.clone(),
                                PlayError::InvalidMove(player, the_move),
                            ));
                        }
                        moves.push(the_move);
                        context.in_progress().add_player_move(player, the_move);
                        context.unset_current_player();
                    }
                    action = next(context, moves);
                }
                Action::Chance { distribution, next } => {}
                Action::Payoff { payoff, next } => {}
            }
        }
    }
}
