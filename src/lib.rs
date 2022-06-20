use core::hash::Hash;

#[macro_use]
mod macros;

mod concat;
mod grouping;
mod memory;
mod unique;

use crate::concat::*;
use crate::grouping::*;
use crate::memory::*;
use crate::unique::*;

pub trait Rollercoaster: Iterator
where
    Self: Sized,
{
    add_exts!(memory, grouping, concat, unique);
}

impl<T: Iterator> Rollercoaster for T {}
