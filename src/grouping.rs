use crate::{memory::Memory, Rollercoaster};

pub trait GroupKind: PartialEq + Copy {}
impl<T> GroupKind for T where T: PartialEq + Copy {}

pub struct GroupBy<I, P>
where
    I: Iterator,
{
    underlying: Memory<I>,
    predicate: P,
}

impl<I, P> GroupBy<I, P>
where
    I: Iterator,
{
    pub(crate) fn new(iter: I, predicate: P) -> Self {
        Self {
            underlying: iter.memory(),
            predicate,
        }
    }
}

pub struct Group<I, K> {
    pub kind: K,
    pub items: Vec<I>,
}

impl<I, P, K> Iterator for GroupBy<I, P>
where
    I: Iterator,
    P: Fn(&I::Item) -> K,
    K: GroupKind,
{
    type Item = Group<I::Item, K>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut accumulation = vec![];
        let mut current_kind = None;

        for item in self.underlying.by_ref() {
            let kind = (self.predicate)(&item);
            let previous_kind = current_kind.unwrap_or(kind);

            if kind != previous_kind {
                self.underlying.remember(item);
                break;
            }

            current_kind = Some(kind);
            accumulation.push(item);
        }

        if accumulation.is_empty() {
            return None;
        }

        Some(Group {
            kind: current_kind.unwrap(),
            items: accumulation,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::Rollercoaster;

    #[test]
    fn it_groups_correctly() {
        let group: Vec<(Vec<i32>, i32)> = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
            .into_iter()
            .group_by(|c| (*c - 1) / 3)
            .map(|g| (g.items, g.kind))
            .collect();

        assert_eq!(
            group,
            vec![
                (vec![1, 2, 3], 0),
                (vec![4, 5, 6], 1),
                (vec![7, 8, 9], 2),
                (vec![10], 3),
            ]
        );
    }
}
