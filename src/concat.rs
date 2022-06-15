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

#[cfg(test)]
mod test {
    use super::{Concat, ConcatSide};

    #[test]
    fn it_concats_both_ways() {
        let a = vec![1, 2, 3].into_iter();
        let b = vec![4, 5, 6].into_iter();
        let c = vec![7, 8, 9].into_iter();

        let iter = Concat::new(b, a, ConcatSide::Start);
        let iter = Concat::new(iter, c, ConcatSide::End);

        let result: Vec<_> = iter.collect();

        assert_eq!(result, vec![1, 2, 3, 4, 5, 6, 7, 8, 9])
    }
}
