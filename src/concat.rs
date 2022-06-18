pub enum ConcatSide {
    Start,
    End,
}

pub struct Concat<I, A> {
    side: ConcatSide,
    underlying: I,
    items: A,
}

impl<I, A> Concat<I, A>
where
    I: Iterator,
    A: Iterator<Item = I::Item>,
{
    pub(crate) fn new(iter: I, items: A, side: ConcatSide) -> Self {
        Self {
            underlying: iter,
            items,
            side,
        }
    }
}

impl<I, A> Iterator for Concat<I, A>
where
    I: Iterator,
    A: Iterator<Item = I::Item>,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.side {
            ConcatSide::Start => self.items.next().or_else(|| self.underlying.next()),
            ConcatSide::End => self.underlying.next().or_else(|| self.items.next()),
        }
    }
}

ext_impl! {
    /**
    Creates an iterator that appends another iterator
    to the end of the existing iterator.

    # Example
    ```
    # use rollercoaster::Rollercoaster;
    #
    let a = vec!["monads", "are", "just", "monoids"];
    let b = vec!["in", "the", "category", "of", "endofunctors"];

    let result: Vec<_> = a
        .into_iter()
        .append(b.into_iter())
        .collect();

    assert_eq!(
        result.join(" "),
        "monads are just monoids in the category of endofunctors".to_string()
    );
    ```
    */
    fn append<I: Iterator<Item = Self::Item>>(self, items: I) -> Concat<Self, I> {
        Concat::new(self, items, ConcatSide::End)
    }

    /**
    Creates an iterator that prepends another iterator
    to the start of the existing iterator. This is the same as `items.append(self)`

    # Example
    ```
    # use rollercoaster::Rollercoaster;
    #
    let a = vec!["opposite", "day"];
    let yoda = vec!["it", "is"];

    let result: Vec<_> = a
        .into_iter()
        .prepend(yoda.into_iter())
        .collect();

    assert_eq!(result.join(" "), "it is opposite day".to_string());
    ```
    */
    fn prepend<I: Iterator<Item = Self::Item>>(self, items: I) -> Concat<Self, I> {
        Concat::new(self, items, ConcatSide::Start)
    }

}

#[cfg(test)]
mod test {
    use crate::Rollercoaster;

    #[test]
    fn it_concats_both_ways() {
        let a = vec![1, 2, 3].into_iter();
        let b = vec![4, 5, 6].into_iter();
        let c = vec![7, 8, 9].into_iter();

        let result: Vec<_> = b.prepend(a).append(c).collect();
        assert_eq!(result, vec![1, 2, 3, 4, 5, 6, 7, 8, 9])
    }
}
