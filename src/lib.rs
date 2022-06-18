#[macro_use]
mod macros;

mod concat;
mod grouping;
mod memory;

use crate::concat::*;
use crate::grouping::*;
use crate::memory::*;

pub trait Rollercoaster: Iterator
where
    Self: Sized,
{
    add_exts!(memory, grouping, concat);
}

impl<T: Iterator> Rollercoaster for T {}
