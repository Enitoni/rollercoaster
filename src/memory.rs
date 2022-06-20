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
    /// Remember this item for the next iteration.
    pub fn remember(&mut self, item: I::Item) {
        self.items.push(item);
    }

    /// Clears all items that were remembered,
    /// and the iterator will now return items
    /// from the underlying iterator.
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Removes the most recently remembered item.
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

ext_impl! {
    /// Creates an iterator that allows remembering one or more values
    /// for the next iterations.
    ///
    /// ## Why?
    /// Sometimes you are working with an iterator in a way
    /// where you need to own values, and at the same time
    /// conditionally perform some action on them across iterations.
    ///
    /// As an example, this is used in
    /// [`group_by()`](crate::Rollercoaster::group_by) to allow lazy grouping.
    ///
    /// ## How is this different from `peekable()`?
    /// Unlike [`Peekable`](std::iter::Peekable),
    /// it allows you to own the value from the current iteration
    /// and choose to use it again in the next one.
    ///
    /// Memory also works in the opposite way, there is no peeking
    /// and instead you can check some condition in the _next_ iteration.
    ///
    /// # Example
    /// ```
    /// # use rollercoaster::Rollercoaster;
    /// #
    /// let mut nums = vec![1, 2, 3, 4, 5, 3].into_iter().memory();
    ///
    /// for n in nums.by_ref() {
    ///     // When 4 is encountered, we want to sum it with
    ///     // everything after it.
    ///     if n == 4 {
    ///         nums.remember(n);
    ///         break;
    ///     }
    /// }
    ///
    /// let summed: u32 = nums.sum();
    /// assert_eq!(summed, 12);
    /// ```
    fn memory(self) -> Memory<Self> {
        Memory::new(self)
    }
}

#[cfg(test)]
mod test {
    use crate::Rollercoaster;

    fn mock() -> Vec<&'static str> {
        vec!["a", "b", "c", "d", "e"]
    }

    #[test]
    fn it_remembers() {
        let mut memory = mock().into_iter().memory();

        for letter in memory.by_ref() {
            if letter == "d" {
                memory.remember(letter);
                break;
            }
        }

        assert_eq!(memory.collect::<Vec<_>>(), vec!["d", "e"]);
    }

    #[test]
    fn it_forgets() {
        let mut memory = mock().into_iter().memory();

        memory.remember("f");
        memory.remember("g");
        memory.forget();

        assert_eq!(
            memory.collect::<Vec<_>>(),
            vec!["f", "b", "b", "c", "d", "e"]
        );
    }
}
