mod memory;

mod rollercoaster {
    use crate::memory::*;

    pub trait Rollercoaster: Iterator
    where
        Self: Sized,
    {
        fn memory(self) -> Memory<Self> {
            Memory::new(self)
        }
    }
}

pub use rollercoaster::Rollercoaster;
