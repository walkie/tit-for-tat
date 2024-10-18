use crate::Utility;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;

/// The cumulative utility for each player across all matchups in a tournament.
///
/// ```
/// use t4t::Score;
///
/// let mut score = Score::new();
///
/// score.add("Leela", 5);
/// score.add("Fry", 3);
/// score.add("Bender", 4);
/// score.add("Leela", 2);
/// score.add("Fry", -5);
///
/// assert_eq!(
///     score.best_to_worst().collect::<Vec<_>>(),
///     vec![
///         ("Leela", 7),
///         ("Bender", 4),
///         ("Fry", -2),
///     ],
/// );
///
/// score.add("Bender", 4);
///
/// assert_eq!(
///     score.worst_to_best().collect::<Vec<_>>(),
///     vec![
///         ("Fry", -2),
///         ("Leela", 7),
///         ("Bender", 8),
///     ],
/// );
/// ```
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Score<U: Utility>(HashMap<String, U>);

impl<U: Utility> Score<U> {
    /// Create an empty score tracker.
    pub fn new() -> Self {
        Score(HashMap::new())
    }

    /// Add a utility value to the given player's current score.
    ///
    /// A player's current score is considered to be zero if they don't have a score yet.
    pub fn add(&mut self, name: &str, utility: U) {
        let current_score = *self.0.get(name).unwrap_or(&U::zero());
        self.0.insert(name.to_owned(), current_score + utility);
    }

    /// Add all scores from another score tracker to this one.
    ///
    /// This is useful for combining scores from multiple tournaments.
    pub fn add_all(&mut self, other: &Score<U>) {
        for (name, score) in &other.0 {
            self.add(name, *score);
        }
    }

    /// Get the current score for the given player.
    pub fn get(&self, name: &str) -> Option<U> {
        self.0.get(name).copied()
    }

    /// Get the current score for the given player, or zero if they don't have a score yet.
    pub fn get_or_zero(&self, name: &str) -> U {
        self.get(name).unwrap_or(U::zero())
    }

    /// Get the players with their associated scores sorted from best (highest score) to worst
    /// (lowest score).
    pub fn best_to_worst(&self) -> impl Iterator<Item = (&str, U)> {
        self.0
            .iter()
            .map(|(name, score)| (name.as_str(), *score))
            .sorted_by(|a, b| PartialOrd::partial_cmp(&b.1, &a.1).unwrap_or(Ordering::Equal))
    }

    /// Get the players with their associated scores sorted from worst (lowest score) to best
    /// (highest score).
    pub fn worst_to_best(&self) -> impl Iterator<Item = (&str, U)> {
        self.0
            .iter()
            .map(|(name, score)| (name.as_str(), *score))
            .sorted_by(|a, b| PartialOrd::partial_cmp(&a.1, &b.1).unwrap_or(Ordering::Equal))
    }

    /// Print the score and player name of each player, from [best to worst](Self::best_to_worst).
    pub fn print_best_to_worst(&self) {
        for (name, score) in self.best_to_worst() {
            println!("{:?}: {}", score, name);
        }
    }

    /// Print the score and player name of each player, from [worst to best](Self::worst_to_best).
    pub fn print_worst_to_best(&self) {
        for (name, score) in self.worst_to_best() {
            println!("{:?}: {}", score, name);
        }
    }
}
