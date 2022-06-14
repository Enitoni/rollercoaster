mod grouping;
mod memory;

use crate::grouping::*;
use crate::memory::*;

pub trait Rollercoaster: Iterator
where
    Self: Sized,
{
    fn memory(self) -> Memory<Self> {
        Memory::new(self)
    }

    fn group_by<P, K>(self, predicate: P) -> GroupBy<Self, P>
    where
        P: Fn(Self::Item) -> K,
        K: GroupKind,
    {
        GroupBy::new(self, predicate)
    }
}

impl<T: Iterator> Rollercoaster for T {}
