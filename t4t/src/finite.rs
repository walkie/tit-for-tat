use crate::{Distribution, Game, PlayerIndex, PossibleMoves, State, Strategy};

/// In a finite game, there is a finite set of moves available on each turn.
///
/// Note that there are two definitions of "finite game" in game theory. The more common definition
/// is a game with a finite number of turns, which is not the definition used here.
pub trait Finite<const P: usize>: Game<P> {
    /// Get an iterator over the moves available to the given player from the given game state.
    fn possible_moves(
        &self,
        player: PlayerIndex<P>,
        state: &Self::State,
    ) -> PossibleMoves<'_, Self::Move>;
}

/// For a finite perfect-information game, produce a strategy that chooses a move randomly from the
/// set of possible moves.
///
/// # Panics
///
/// Panics if the number of possible moves is 0 or larger than `u32::MAX`.
pub fn randomly<S, G, const P: usize>(game: &'static G) -> Strategy<S, G::Move, P>
where
    S: State,
    G: Finite<P> + Game<P, State = S, View = S>,
{
    Strategy::new(|context| {
        let state = context.state_view();
        let player = context.my_index();
        let moves = game.possible_moves(player, state).collect::<Vec<_>>();
        let dist = Distribution::flat(moves);
        match dist {
            Some(dist) => dist.sample().to_owned(),
            None => panic!("randomly: Could not build distribution."),
        }
    })
}
