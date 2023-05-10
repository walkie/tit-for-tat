use crate::error::Error;
use crate::moves::Move;
use crate::payoff::{Payoff, Utility};
use crate::per_player::{PerPlayer, PlayerIndex};

use crate::sim::context::Context;
use crate::sim::outcome::Outcome;
use crate::sim::profile::Profile;

/// A player of a simultaneous game. Consists of a name and a [strategy](crate::Strategy).
pub type Player<G, const P: usize> =
    crate::Player<Context<<G as Game<P>>::Move, <G as Game<P>>::Utility, P>, <G as Game<P>>::Move>;

/// A [per-player](crate::PerPlayer) collection of simultaneous game [players](Player).
pub type Players<G, const P: usize> = PerPlayer<Player<G, P>, P>;

/// Result of playing a game. Eite
// pub type PlayResult<T, G, const P: usize> = Result<T, Error<<G as Game<P>>::Move, P>>;

/// A simultaneous game.
pub trait Game<const P: usize>: Sized {
    // TODO: Someday, when the assocated_const_equality and/or generic_const_exprs features are
    // implemented, replace this trait's const generic P with the following associated constant.
    // const PLAYERS: usize;

    /// The type of moves played by players in this game.
    type Move: Move;

    /// The type of utility values awarded to each player at the end of the game.
    type Utility: Utility;

    /// Get the payoff for the given strategy profile.
    ///
    /// This method may return an arbitrary payoff if given an
    /// [invalid profile](Simultaneous::is_valid_profile).
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
        context: &'c mut Context<Self::Move, Self::Utility, P>
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

         Ok(context.complete(profile, self.payoff(profile)))
    }

    /// Play a game once with the given players, returning the outcome if successful.
    #[allow(clippy::type_complexity)]
    fn play_once(
        &self,
        players: &mut Players<Self, P>
    ) -> Result<Outcome<Self::Move, Self::Utility, P>, Error<Self::Move, P>> {
        let mut context = Context::new();
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
    ) -> Result<Context<Self::Move, Self::Utility, P>, Error<Self::Move, P>> {
        let mut context = Context::new();
        for _ in 0..iterations {
           self.play_in_context(players, &mut context)?;
        }
        Ok(context)
    }
}
