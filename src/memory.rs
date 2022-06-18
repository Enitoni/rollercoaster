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
    /// # use rollercoaster::Rollercoaster;
    /// #
    /// let mut items = vec![
    ///     "apple",
    ///     "banana",
    ///     "orange"
    /// ].into_iter().memory();
    ///
    /// for fruit in items.by_ref() {
    ///     if fruit.starts_with("o") {
    ///         // Remember orange for next iteration
    ///         items.remember(fruit);
    ///
    ///         // Prevent infinite loop
    ///         break;
    ///     }
    /// }
    ///
    /// assert_eq!(items.next(), Some("orange"));
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

ext_impl! {
    /**
    Creates an iterator that allows remembering values
    for the next iteration.

    This is useful in situations where you need to:
    - Read from the item, then use it in the next iteration.
    - Insert items returned by the iterator.

    ## How is this different from `peekable()`?
    [Memory] is more flexible because unlike `Peekable`,
    it allows you to own the value you are working with
    and decide if you need to use it the next iteration.

    Since `remember()` only takes owned values, you avoid
    dealing with references and it can make the iterator easier to work with.

    # Example
    ```
    # use rollercoaster::Rollercoaster;
    #
    let mut nums = vec![1, 2, 3, 4, 5].into_iter().memory();

    for n in nums.by_ref() {
        if n == 4 {
            nums.remember(n);
            break;
        }
    }

    let summed: u32 = nums.sum();
    assert_eq!(summed, 9);
    ```
    */
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
            vec!["f", "a", "b", "c", "d", "e"]
        );
    }
}
