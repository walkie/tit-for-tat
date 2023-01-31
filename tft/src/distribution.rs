use rand_distr::WeightedAliasIndex;

/// A weighted probability distribution over a set of discrete elements, such as moves.
///
/// A distribution consists of a set of elements with associated weights. A weight indicates how
/// likely that element is compared to the other elements in the distribution.
///
/// # Examples
///
/// The following distribution defines a weighted coin where heads is three times as likely to
/// occur as tails. The value `coin` can be expected to be `"heads"` 75% of the time and `"tails"`
/// 25% of the time.
///
/// ```
/// use tft::Distribution;
///
/// let dist = Distribution::new(vec![("heads", 3.0), ("tails", 1.0)]).unwrap();
/// let coin = dist.sample();
/// ```
///
/// In the following distribution, the value `'A'` is 2.5 times as likely as `'B'` and 5 times as
/// likely as `'C'`, so `abc` can be expected to be `'A'` 62.5% (5/8) of the time, `'B'` 25% of the
/// time, and `'C'` 12.5% of the time.
///
/// ```
/// use tft::Distribution;
///
/// let dist = Distribution::new(vec![('A', 2.5), ('B', 1.0), ('C', 0.5)]).unwrap();
/// let abc = dist.sample();
/// ```
pub struct Distribution<T> {
    elements: Vec<T>,
    dist: WeightedAliasIndex<f64>,
}

impl<T> Distribution<T> {
    /// Create a new weighted distribution given an association list of elements and their weights.
    ///
    /// # Errors
    ///
    /// Logs an error and returns `None` if:
    /// - The vector is empty.
    /// - The vector is longer than u32::MAX.
    /// - For any weight `w`: `w < 0.0` or `w > max`
    ///   where `max = f64::MAX / weighted_elements.len()`.
    /// - The sum of the weights is zero.
    pub fn new(weighted_elements: Vec<(T, f64)>) -> Option<Self> {
        let (elements, weights) = weighted_elements.into_iter().unzip();
        match WeightedAliasIndex::new(weights) {
            Ok(dist) => Some(Distribution { elements, dist }),
            Err(err) => {
                log::error!(
                    "Distribution::new: Error creating weighted probability distribution: {:?}",
                    err
                );
                None
            }
        }
    }

    /// Sample a random value from the distribution using `rng` as the source of randomness.
    pub fn sample_using<R: rand::Rng>(&self, rng: &mut R) -> &T {
        let index =
            <WeightedAliasIndex<f64> as rand_distr::Distribution<usize>>::sample(&self.dist, rng);
        &self.elements[index]
    }

    /// Sample a random value from the distribution using `rand::thread_rng()` as the source of
    /// randomness.
    pub fn sample(&self) -> &T {
        self.sample_using(&mut rand::thread_rng())
    }
}
