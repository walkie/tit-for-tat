use crate::{Game, Matchup, Outcome, PerPlayer, PlayResult, Player};
use itertools::Itertools;
use log::error;
use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

/// A tournament in which several players play a game against each other in a series of matchups.
#[derive(Clone, Debug)]
pub struct Tournament<G: Game<P>, const P: usize> {
    game: Arc<G>,
    matchups: Vec<Matchup<G, P>>,
}

/// Build a tournament by generating and filtering the matchups.
pub struct TournamentBuilder<G: Game<P>, const P: usize> {
    game: Arc<G>,
    matchups: Vec<Matchup<G, P>>,
}

/// The collected results from running a tournament.
#[derive(Clone, Debug, PartialEq)]
pub struct TournamentResult<G: Game<P>, const P: usize> {
    results: HashMap<PerPlayer<String, P>, PlayResult<G, P>>,
    scores: HashMap<String, G::Utility>,
    has_errors: bool,
}

impl<G: Game<P>, const P: usize> Tournament<G, P> {
    /// Construct a new tournament for the given game with the given list of matchups.
    pub fn new(game: Arc<G>, matchups: Vec<Matchup<G, P>>) -> Self {
        Tournament { game, matchups }
    }

    /// Construct a new tournament where the matchups are all
    /// [combinations](https://en.wikipedia.org/wiki/Combination)
    /// [with replacement](https://en.wikipedia.org/wiki/Sampling_(statistics)#Replacement_of_selected_units)
    /// of the given list of players.
    ///
    /// That is, all selections of players where the order does not matter and the same player may
    /// be repeated within a matchup.
    ///
    /// # Example
    /// ```
    /// use std::sync::Arc;
    /// use t4t::*;
    ///
    /// let players = vec!["A", "B", "C"]
    ///     .into_iter()
    ///     .map(|name| Arc::new(Player::new(name.to_string(), || Strategy::pure(()))))
    ///     .collect();
    ///
    /// let game: Simultaneous<(), u8, 3> = Simultaneous::trivial();
    ///
    /// let tournament = Tournament::combinations_with_replacement(
    ///     Arc::new(game),
    ///     players,
    /// );
    ///
    /// assert_eq!(
    ///     tournament
    ///         .matchups()
    ///         .iter()
    ///         .map(|m| m.names())
    ///         .collect::<Vec<PerPlayer<String, 3>>>(),
    ///     vec![
    ///         PerPlayer::new(["A", "A", "A"]).map(|s| s.to_string()),
    ///         PerPlayer::new(["A", "A", "B"]).map(|s| s.to_string()),
    ///         PerPlayer::new(["A", "A", "C"]).map(|s| s.to_string()),
    ///         PerPlayer::new(["A", "B", "B"]).map(|s| s.to_string()),
    ///         PerPlayer::new(["A", "B", "C"]).map(|s| s.to_string()),
    ///         PerPlayer::new(["A", "C", "C"]).map(|s| s.to_string()),
    ///         PerPlayer::new(["B", "B", "B"]).map(|s| s.to_string()),
    ///         PerPlayer::new(["B", "B", "C"]).map(|s| s.to_string()),
    ///         PerPlayer::new(["B", "C", "C"]).map(|s| s.to_string()),
    ///         PerPlayer::new(["C", "C", "C"]).map(|s| s.to_string()),
    ///     ],
    /// )
    /// ```
    pub fn combinations_with_replacement(game: Arc<G>, players: Vec<Arc<Player<G, P>>>) -> Self {
        Tournament::new(
            game,
            players
                .into_iter()
                .combinations_with_replacement(P)
                .map(|player_vec| Matchup::new(PerPlayer::new(player_vec.try_into().unwrap())))
                .collect(),
        )
    }

    /// Construct a new tournament where the matchups are all
    /// [combinations](https://en.wikipedia.org/wiki/Combination)
    /// [without replacement](https://en.wikipedia.org/wiki/Sampling_(statistics)#Replacement_of_selected_units)
    /// of the given list of players.
    ///
    /// That is, all selections of players where the order does not matter and all players are
    /// distinct within a matchup.
    ///
    /// # Example
    /// ```
    /// use std::sync::Arc;
    /// use t4t::*;
    ///
    /// let players = vec!["A", "B", "C", "D", "E"]
    ///     .into_iter()
    ///     .map(|name| Arc::new(Player::new(name.to_string(), || Strategy::pure(()))))
    ///     .collect();
    ///
    /// let game: Simultaneous<(), u8, 3> = Simultaneous::trivial();
    ///
    /// let tournament = Tournament::combinations_without_replacement(
    ///     Arc::new(game),
    ///     players,
    /// );
    ///
    /// assert_eq!(
    ///     tournament
    ///         .matchups()
    ///         .iter()
    ///         .map(|m| m.names())
    ///         .collect::<Vec<PerPlayer<String, 3>>>(),
    ///     vec![
    ///         PerPlayer::new(["A", "B", "C"]).map(|s| s.to_string()),
    ///         PerPlayer::new(["A", "B", "D"]).map(|s| s.to_string()),
    ///         PerPlayer::new(["A", "B", "E"]).map(|s| s.to_string()),
    ///         PerPlayer::new(["A", "C", "D"]).map(|s| s.to_string()),
    ///         PerPlayer::new(["A", "C", "E"]).map(|s| s.to_string()),
    ///         PerPlayer::new(["A", "D", "E"]).map(|s| s.to_string()),
    ///         PerPlayer::new(["B", "C", "D"]).map(|s| s.to_string()),
    ///         PerPlayer::new(["B", "C", "E"]).map(|s| s.to_string()),
    ///         PerPlayer::new(["B", "D", "E"]).map(|s| s.to_string()),
    ///         PerPlayer::new(["C", "D", "E"]).map(|s| s.to_string()),
    ///     ],
    /// )
    /// ```
    pub fn combinations_without_replacement(game: Arc<G>, players: Vec<Arc<Player<G, P>>>) -> Self {
        Tournament::new(
            game,
            players
                .into_iter()
                .combinations(P)
                .map(|player_vec| Matchup::new(PerPlayer::new(player_vec.try_into().unwrap())))
                .collect(),
        )
    }

    /// Construct a new tournament where the matchups are all
    /// [premutations](https://en.wikipedia.org/wiki/Permutation)
    /// [with replacement](https://en.wikipedia.org/wiki/Sampling_(statistics)#Replacement_of_selected_units)
    /// of the given list of players.
    ///
    /// That is, all orderings of all selections of players where the same player may be repeated
    /// within in a matchup.
    ///
    /// # Example
    /// ```
    /// use std::sync::Arc;
    /// use t4t::*;
    ///
    /// let players = vec!["A", "B"]
    ///     .into_iter()
    ///     .map(|name| Arc::new(Player::new(name.to_string(), || Strategy::pure(()))))
    ///     .collect();
    ///
    /// let game: Simultaneous<(), u8, 3> = Simultaneous::trivial();
    ///
    /// let tournament = Tournament::permutations_with_replacement(
    ///     Arc::new(game),
    ///     players,
    /// );
    ///
    /// assert_eq!(
    ///     tournament
    ///         .matchups()
    ///         .iter()
    ///         .map(|m| m.names())
    ///         .collect::<Vec<PerPlayer<String, 3>>>(),
    ///     vec![
    ///         PerPlayer::new(["A", "A", "A"]).map(|s| s.to_string()),
    ///         PerPlayer::new(["A", "A", "B"]).map(|s| s.to_string()),
    ///         PerPlayer::new(["A", "B", "A"]).map(|s| s.to_string()),
    ///         PerPlayer::new(["A", "B", "B"]).map(|s| s.to_string()),
    ///         PerPlayer::new(["B", "A", "A"]).map(|s| s.to_string()),
    ///         PerPlayer::new(["B", "A", "B"]).map(|s| s.to_string()),
    ///         PerPlayer::new(["B", "B", "A"]).map(|s| s.to_string()),
    ///         PerPlayer::new(["B", "B", "B"]).map(|s| s.to_string()),
    ///     ],
    /// )
    /// ```
    pub fn permutations_with_replacement(game: Arc<G>, players: Vec<Arc<Player<G, P>>>) -> Self {
        Tournament::new(
            game,
            itertools::repeat_n(players, P)
                .multi_cartesian_product()
                .map(|player_vec| Matchup::new(PerPlayer::new(player_vec.try_into().unwrap())))
                .collect(),
        )
    }

    /// Construct a new tournament where the matchups are all
    /// [premutations](https://en.wikipedia.org/wiki/Permutation)
    /// [without replacement](https://en.wikipedia.org/wiki/Sampling_(statistics)#Replacement_of_selected_units)
    /// of the given list of players.
    ///
    /// That is, all orderings of all selections of players where players are distinct within a
    /// matchup.
    ///
    /// # Example
    /// ```
    /// use std::sync::Arc;
    /// use t4t::*;
    ///
    /// let players = vec!["A", "B", "C"]
    ///     .into_iter()
    ///     .map(|name| Arc::new(Player::new(name.to_string(), || Strategy::pure(()))))
    ///     .collect();
    ///
    /// let game: Simultaneous<(), u8, 2> = Simultaneous::trivial();
    ///
    /// let tournament = Tournament::permutations_without_replacement(
    ///     Arc::new(game),
    ///     players,
    /// );
    ///
    /// assert_eq!(
    ///     tournament
    ///         .matchups()
    ///         .iter()
    ///         .map(|m| m.names())
    ///         .collect::<Vec<PerPlayer<String, 2>>>(),
    ///     vec![
    ///         PerPlayer::new(["A", "B"]).map(|s| s.to_string()),
    ///         PerPlayer::new(["A", "C"]).map(|s| s.to_string()),
    ///         PerPlayer::new(["B", "A"]).map(|s| s.to_string()),
    ///         PerPlayer::new(["B", "C"]).map(|s| s.to_string()),
    ///         PerPlayer::new(["C", "A"]).map(|s| s.to_string()),
    ///         PerPlayer::new(["C", "B"]).map(|s| s.to_string()),
    ///     ],
    /// )
    /// ```
    pub fn permutations_without_replacement(game: Arc<G>, players: Vec<Arc<Player<G, P>>>) -> Self {
        Tournament::new(
            game,
            players
                .into_iter()
                .permutations(P)
                .map(|player_vec| Matchup::new(PerPlayer::new(player_vec.try_into().unwrap())))
                .collect(),
        )
    }

    /// Construct a new tournament where the matchups are the cartesian product of the given
    /// per-player collection of lists of players, that is, every combination where one player is
    /// drawn from each list.
    ///
    /// This constructor is well-suited to non-symmetric games where different groups of players
    /// are specific to different roles in the game.
    ///
    /// # Example
    /// ```
    /// use std::sync::Arc;
    /// use t4t::*;
    ///
    /// let [a, b, c, d, e, f, g] = ["A", "B", "C", "D", "E", "F", "G"]
    ///     .map(|name| Arc::new(Player::new(name.to_string(), || Strategy::pure(()))));    ///
    ///
    /// let game: Simultaneous<(), u8, 3> = Simultaneous::trivial();
    ///
    /// let tournament = Tournament::product(
    ///     Arc::new(game),
    ///     PerPlayer::new([vec![a, b, c], vec![d, e], vec![f, g]])
    /// );
    ///
    /// assert_eq!(
    ///     tournament
    ///         .matchups()
    ///         .iter()
    ///         .map(|m| m.names())
    ///         .collect::<Vec<PerPlayer<String, 3>>>(),
    ///     vec![
    ///         PerPlayer::new(["A", "D", "F"]).map(|s| s.to_string()),
    ///         PerPlayer::new(["A", "D", "G"]).map(|s| s.to_string()),
    ///         PerPlayer::new(["A", "E", "F"]).map(|s| s.to_string()),
    ///         PerPlayer::new(["A", "E", "G"]).map(|s| s.to_string()),
    ///         PerPlayer::new(["B", "D", "F"]).map(|s| s.to_string()),
    ///         PerPlayer::new(["B", "D", "G"]).map(|s| s.to_string()),
    ///         PerPlayer::new(["B", "E", "F"]).map(|s| s.to_string()),
    ///         PerPlayer::new(["B", "E", "G"]).map(|s| s.to_string()),
    ///         PerPlayer::new(["C", "D", "F"]).map(|s| s.to_string()),
    ///         PerPlayer::new(["C", "D", "G"]).map(|s| s.to_string()),
    ///         PerPlayer::new(["C", "E", "F"]).map(|s| s.to_string()),
    ///         PerPlayer::new(["C", "E", "G"]).map(|s| s.to_string()),
    ///     ],
    /// )
    /// ```
    pub fn product(game: Arc<G>, players_per_slot: PerPlayer<Vec<Arc<Player<G, P>>>, P>) -> Self {
        Tournament::new(
            game,
            players_per_slot
                .into_iter()
                .multi_cartesian_product()
                .map(|player_vec| Matchup::new(PerPlayer::new(player_vec.try_into().unwrap())))
                .collect(),
        )
    }

    /// Run the tournament and collect the results.
    pub fn play(&self) -> TournamentResult<G, P> {
        let mut results = HashMap::new();
        let mut scores = HashMap::new();
        let mut has_errors = false;

        let (sender, receiver) = std::sync::mpsc::channel();

        self.matchups
            .par_iter()
            .for_each_with(sender, |s, matchup| {
                let result = self.game.play(matchup);
                let send_result = s.send((matchup.names(), result));
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

    /// Get a reference to the game being played in this tournament.
    pub fn game(&self) -> &Arc<G> {
        &self.game
    }

    /// Get all the matchups in this tournament.
    pub fn matchups(&self) -> &Vec<Matchup<G, P>> {
        &self.matchups
    }
}

impl<G: Game<P> + 'static, const P: usize> TournamentBuilder<G, P> {
    /// Construct a new tournament builder for the given game.
    pub fn new(game: Arc<G>) -> Self {
        TournamentBuilder {
            game,
            matchups: Vec::new(),
        }
    }

    /// Add a matchup to the tournament.
    pub fn add_matchup(&mut self, matchup: Matchup<G, P>) {
        self.matchups.push(matchup);
    }

    /// Build the tournament from the added matchups.
    pub fn build(self) -> Tournament<G, P> {
        Tournament::new(self.game, self.matchups)
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
