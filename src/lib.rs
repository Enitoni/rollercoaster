mod concat;
mod grouping;
mod memory;

use crate::grouping::*;
use crate::memory::*;

pub trait Rollercoaster: Iterator
where
    Self: Sized,
{
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

    /**
    Creates an iterator that group items by a predicate.

    The value returned by the predicate decides
    which group the set of items belong to.

    Note that a new group is created each time the value
    returned by the predicate changes. It does not group all items
    into a set of specific groups.

    # Example
    ```
    # use rollercoaster::Rollercoaster;
    #
    let words = vec!["super", "sad", "mega", "rude", "cool", "sand"];

    let grouped: Vec<_> = words
        .into_iter()
        .group_by(|w| w.starts_with("s"))
        .map(|g| g.items)
        .collect();

    assert_eq!(grouped, vec![
        vec!["super", "sad"],
        vec!["mega", "rude", "cool"],
        vec!["sand"],
    ]);
    ```
    */
    fn group_by<P, K>(self, predicate: P) -> GroupBy<Self, P>
    where
        P: Fn(&Self::Item) -> K,
        K: GroupKind,
    {
        GroupBy::new(self, predicate)
    }
}

impl<T: Iterator> Rollercoaster for T {}
