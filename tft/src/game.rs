use crate::{
    Context, Error, Kind, Move, Outcome, Payoff, PerPlayer, PlayerIndex, Players, Profile, Sim,
    Utility,
};

/// The main interface for playing games.
pub trait Game<const P: usize>: Sized {
    type Kind: Kind;

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

    /// The initial game state.
    fn initial_state(&self) -> Self::State;

    /// Is this a valid move at the given state for the given player?
    fn is_valid_move_from_state(
        &self,
        state: &Self::State,
        player: PlayerIndex<P>,
        the_move: Self::Move,
    ) -> bool;

    fn is_sequential(&self) -> bool {
        Self::Kind::is_sequential()
    }

    fn is_simultaneous(&self) -> bool {
        Self::Kind::is_simultaneous()
    }

    /// The number of players this game is for.
    fn num_players(&self) -> usize {
        P
    }
}

/// The main interface for playing simultaneous games.
pub trait Simultaneous<const P: usize>: Game<P, Kind = Sim, State = ()> {
    /// Get the payoff for the given strategy profile.
    ///
    /// This method may return an arbitrary payoff if given an
    /// [invalid profile](crate::Simultaneous::is_valid_profile).
    fn payoff(&self, profile: Profile<Self::Move, P>) -> Payoff<Self::Utility, P>;

    /// Is this a valid move for the given player?
    fn is_valid_move(&self, player: PlayerIndex<P>, the_move: Self::Move) -> bool {
        self.is_valid_move_from_state(&(), player, the_move)
    }

    /// Is this a valid strategy profile? A profile is valid if each move is valid for the
    /// corresponding player.
    fn is_valid_profile(&self, profile: Profile<Self::Move, P>) -> bool {
        PlayerIndex::all_indexes().all(|pi| self.is_valid_move(pi, profile[pi]))
    }

    /// Play one iteration of the game in the given context. Update the context and return the
    /// outcome if successful.
    #[allow(clippy::type_complexity)]
    fn play_in_context<'c>(
        &self,
        players: &mut Players<Self, P>,
        context: &'c mut Context<Self, P>,
    ) -> Result<&'c Outcome<Sim, Self::Move, Self::Utility, P>, Error<Self::Move, P>> {
        let profile = PerPlayer::generate(|i| {
            context.set_current_player(i);
            players[i].next_move(context)
        });
        context.unset_current_player();

        for i in PlayerIndex::all_indexes() {
            if !self.is_valid_move(i, profile[i]) {
                return Err(Error::InvalidMove(i, profile[i]));
            }
        }

        Ok(context.complete(Outcome::new(profile, self.payoff(profile))))
    }

    /// Play a game once with the given players, returning the outcome if successful.
    #[allow(clippy::type_complexity)]
    fn play_once(
        &self,
        players: &mut Players<Self, P>,
    ) -> Result<Outcome<Sim, Self::Move, Self::Utility, P>, Error<Self::Move, P>> {
        let mut context = Context::new(());
        let outcome = self.play_in_context(players, &mut context)?;
        Ok(outcome.clone())
    }

    /// Play a given number of iterations of a game with the given players, starting from the
    /// initial state. Return the final context if successful.
    #[allow(clippy::type_complexity)]
    fn play_repeatedly(
        &self,
        iterations: u32,
        players: &mut Players<Self, P>,
    ) -> Result<Context<Self, P>, Error<Self::Move, P>> {
        let mut context = Context::new(());
        for _ in 0..iterations {
            self.play_in_context(players, &mut context)?;
        }
        Ok(context)
    }
}
