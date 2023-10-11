use crate::sim::{Outcome, Profile};
use crate::{Context, Error, Game, Payoff, PerPlayer, PlayerIndex, Players};

/// The main interface for playing simultaneous games.
pub trait SimultaneousGame<const P: usize>:
    Game<P, State = (), Record = Outcome<<Self as Game<P>>::Move, <Self as Game<P>>::Utility, P>>
{
    /// Get the payoff for the given strategy profile.
    ///
    /// This method may return an arbitrary payoff if given an
    /// [invalid profile](crate::sim::Simultaneous::is_valid_profile).
    fn payoff(&self, profile: Profile<Self::Move, P>) -> Payoff<Self::Utility, P>;

    /// Is this a valid move for the given player?
    fn is_valid_move(&self, player: PlayerIndex<P>, the_move: Self::Move) -> bool;

    /// Is this a valid strategy profile? A profile is valid if each move is valid for the
    /// corresponding player.
    fn is_valid_profile(&self, profile: Profile<Self::Move, P>) -> bool {
        PlayerIndex::all_indexes().all(|pi| self.is_valid_move(pi, profile[pi]))
    }

    /// The number of players this game is for.
    fn num_players(&self) -> usize {
        P
    }

    /// Play one iteration of the game in the given context. Update the context and return the
    /// outcome if successful.
    #[allow(clippy::type_complexity)]
    fn play_in_context<'c>(
        &self,
        players: &mut Players<Self, P>,
        context: &'c mut Context<Self, P>,
    ) -> Result<&'c Outcome<Self::Move, Self::Utility, P>, Error<Self::Move, P>> {
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
    ) -> Result<Outcome<Self::Move, Self::Utility, P>, Error<Self::Move, P>> {
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
