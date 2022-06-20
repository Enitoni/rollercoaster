use std::{collections::HashSet, hash::Hash};

pub struct Unique<I, K, F> {
    seen: HashSet<K>,
    underlying: I,
    identify: F,
}

impl<I, K, F> Unique<I, K, F>
where
    I: Iterator,
    K: Hash,
    F: Fn(&I::Item) -> K,
{
    pub(crate) fn new(iter: I, identify: F) -> Self {
        Self {
            seen: Default::default(),
            underlying: iter,
            identify,
        }
    }
}

impl<I, K, F> Iterator for Unique<I, K, F>
where
    I: Iterator,
    K: Eq + Hash,
    F: Fn(&I::Item) -> K,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.underlying.find(|i| {
            let key = (self.identify)(&i);
            !self.seen.contains(&key)
        });

        if let Some(i) = item.as_ref() {
            let key = (self.identify)(i);
            self.seen.insert(key);
        }

        item
    }
}

ext_impl! {
    /// Creates an iterator that returns only unique values.
    ///
    /// The closure `F` is called on each item, where the returned value `K`
    /// is checked against a [HashSet](std::collections::HashSet).
    /// This means that `K` must implement
    /// [Hash] + [Eq].
    ///
    /// If you are working with simple values, you can try
    /// using [`unique`](crate::Rollercoaster::unique) instead.
    ///
    /// # Example
    /// ```
    /// # use rollercoaster::Rollercoaster;
    /// let words = vec![
    ///     "I",
    ///     "A",
    ///     "am",
    ///     "no",
    ///     "to",
    ///     "ha",
    ///     "unique",
    ///     "people",
    ///     "me",
    /// ];
    ///
    /// let result: Vec<_> = words
    ///     .into_iter()
    ///     .unique_by(|s| s.len())
    ///     .collect();
    ///
    /// assert_eq!(result, vec!["I", "am", "unique"]);
    /// ```
    fn unique_by<K, F>(self, identify: F) -> Unique<Self, K, F>
    where
        K: Hash + Eq,
        F: Fn(&Self::Item) -> K
    {
        Unique::new(self, identify)
    }


    /// Creates an iterator that returns unique values.
    ///
    /// Each value is cloned and checked against a [HashSet](std::collections::HashSet).
    /// This means [`Item`](Iterator::Item) must implement
    /// [Eq] + [Hash] + [Copy].
    ///
    /// If you need to specify a custom key per item,
    /// use [`unique_by()`](crate::Rollercoaster::unique_by) instead.
    ///
    /// # Example
    /// ```
    /// # use rollercoaster::Rollercoaster;
    /// let result: Vec<_> = vec![1, 1, 4, 5, 2, 1, 4, 3, 2]
    ///     .into_iter()
    ///     .unique()
    ///     .collect();
    ///
    /// assert_eq!(result, vec![1, 4, 5, 2, 3]);
    /// ```
    fn unique(self) -> Unique<Self, Self::Item, fn(&Self::Item) -> Self::Item>
    where
        Self::Item: Hash + Eq + Copy
    {
        Unique::new(self, |i| *i)
    }
}

#[cfg(test)]
mod test {
    use crate::Rollercoaster;

    #[test]
    fn it_returns_unique() {
        let result: Vec<_> = vec![5, 2, 6, 7, 3, 3, 2, 4, 5]
            .into_iter()
            .unique()
            .collect();

        let result_two: Vec<_> = vec!["apple", "apple", "orange"]
            .into_iter()
            .unique_by(|s| s.len())
            .collect();

        assert_eq!(result, vec![5, 2, 6, 7, 3, 4]);
        assert_eq!(result_two, vec!["apple", "orange"]);
    }
}
