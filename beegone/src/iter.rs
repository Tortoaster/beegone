use std::iter::Filter;

use crate::{board::Board, pos::Pos};

pub trait IteratorExt: sealed::Sealed {
    fn within_bounds(self) -> Filter<Self, fn(&Pos) -> bool>
    where
        Self: Sized;
}

impl<I: Iterator<Item = Pos>> IteratorExt for I {
    fn within_bounds(self) -> Filter<Self, fn(&Pos) -> bool> {
        self.filter(|pos| Board::within_bounds(*pos))
    }
}

mod sealed {
    use crate::pos::Pos;

    pub trait Sealed {}

    impl<I: Iterator<Item = Pos>> Sealed for I {}
}
