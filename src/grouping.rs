use crate::memory::Memory;

pub trait GroupKind: PartialEq + Copy {}
impl<T> GroupKind for T where T: PartialEq + Copy {}

pub struct GroupBy<I, P>
where
    I: Iterator,
{
    underlying: Memory<I>,
    predicate: P,
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
    use crate::memory::Memory;

    use super::GroupBy;

    #[test]
    fn it_groups() {
        let mut group = GroupBy {
            predicate: |s: &char| s.is_whitespace(),
            underlying: Memory::new("This is a grouping test!".chars()),
        }
        .map(|g| (g.items.into_iter().collect::<String>(), g.kind));

        assert_eq!(group.next(), Some(("This".to_string(), false)));
        assert_eq!(group.next(), Some((" ".to_string(), true)));
        assert_eq!(group.next(), Some(("is".to_string(), false)));
        assert_eq!(group.next(), Some((" ".to_string(), true)));
        assert_eq!(group.next(), Some(("a".to_string(), false)));
        assert_eq!(group.next(), Some((" ".to_string(), true)));
        assert_eq!(group.next(), Some(("grouping".to_string(), false)));
        assert_eq!(group.next(), Some((" ".to_string(), true)));
        assert_eq!(group.next(), Some(("test!".to_string(), false)));
    }
}
