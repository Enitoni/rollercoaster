pub struct Memory<I>
where
    I: Iterator,
{
    items: Vec<I::Item>,
    underlying: I,
}

impl<I> Memory<I>
where
    I: Iterator,
{
    /// Puts the item into memory, so that on the next iteration
    /// this item is returned.
    ///
    /// # Example
    /// ```
    /// let mut items = vec![
    ///     "apple",
    ///     "banana",
    ///     "orange"
    /// ].iter().memory();
    ///
    /// for fruit in items.by_ref() {
    ///     if fruit.starts_with("o") {
    ///         items.remember(fruit);
    ///         break;
    ///     }
    /// }
    ///
    /// assert_eq!(items.next(), "orange");
    /// ```
    pub fn remember(&mut self, item: I::Item) {
        self.items.push(item);
    }

    /// Clears all items that were remembered,
    /// causing the iterator to return new items.
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Forgets the last remembered item.
    pub fn forget(&mut self) {
        self.items.pop();
    }

    pub(crate) fn new(underlying: I) -> Self {
        Self {
            underlying,
            items: vec![],
        }
    }
}

impl<I> Iterator for Memory<I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.items.is_empty() {
            return self.items.pop();
        }

        self.underlying.next()
    }
}

#[cfg(test)]
mod test {
    use super::Memory;

    fn mock() -> Vec<String> {
        vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
            "e".to_string(),
        ]
    }

    #[test]
    fn it_remembers() {
        let mut memory = Memory::new(mock().into_iter());

        for letter in memory.by_ref() {
            if letter == "d" {
                memory.remember(letter);
                break;
            }
        }

        assert_eq!(memory.next(), Some("d".to_string()));
        assert_eq!(memory.next(), Some("e".to_string()));
    }

    #[test]
    fn it_forgets() {
        let mut memory = Memory::new(mock().into_iter());

        memory.remember("f".to_string());
        memory.remember("g".to_string());
        memory.forget();

        assert_eq!(memory.next(), Some("f".to_string()));
        assert_eq!(memory.next(), Some("a".to_string()));
    }
}
