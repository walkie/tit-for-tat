use crate::{Game, Outcome, PerPlayer, PlayResult, Players};
use log::error;
use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

/// A tournament in which several players play a game against each other in a series of matchups.
pub struct Tournament<G: Game<P>, const P: usize> {
    game: Arc<G>,
    matchups: Vec<Players<G, P>>,
}

/// The collected results from running a tournament.
pub struct TournamentResult<G: Game<P>, const P: usize> {
    results: HashMap<PerPlayer<String, P>, PlayResult<G, P>>,
    scores: HashMap<String, G::Utility>,
    has_errors: bool,
}

impl<G: Game<P>, const P: usize> Tournament<G, P> {
    /// Construct a new tournament for the given game with the given matchups.
    pub fn new(game: Arc<G>, matchups: Vec<Players<G, P>>) -> Self {
        Tournament { game, matchups }
    }

    /// Run the tournament and collect the results.
    pub fn play(&self) -> TournamentResult<G, P> {
        let mut results = HashMap::new();
        let mut scores = HashMap::new();
        let mut has_errors = false;

        let (sender, receiver) = std::sync::mpsc::channel();

        self.matchups
            .par_iter()
            .for_each_with(sender, |s, players| {
                let names = players.map(|player| player.name().to_owned());
                let result = self.game.play(players);
                let send_result = s.send((names, result));
                if let Err(err) = send_result {
                    error!("error sending result: {:?}", err);
                }
            });

        receiver.iter().for_each(|(names, result)| {
            if let Ok(outcome) = &result {
                names.for_each_with_index(|i, name| {
                    scores.insert(name.to_owned(), outcome.payoff()[i]);
                });
            } else {
                has_errors = true;
            }
            results.insert(names, result);
        });

        TournamentResult {
            results,
            scores,
            has_errors,
        }
    }
}

impl<G: Game<P>, const P: usize> TournamentResult<G, P> {
    /// The individual play result of each matchup.
    pub fn results(&self) -> &HashMap<PerPlayer<String, P>, PlayResult<G, P>> {
        &self.results
    }

    /// The cumulative utility for each player across all matchups.
    ///
    /// Note that failed matchups will result in no added utility for either player in the matchup,
    /// so this value should not be relied on if [`has_errors`](Self::has_errors) is true.
    pub fn scores(&self) -> &HashMap<String, G::Utility> {
        &self.scores
    }

    /// Did any of the matchups end in an error rather than a successful outcome?
    pub fn has_errors(&self) -> bool {
        self.has_errors
    }
}
