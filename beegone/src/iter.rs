use crate::board::Board;
use crate::pos::Pos;
use std::iter::Filter;

pub trait IteratorExt: sealed::Sealed {
    fn on_board(self) -> Filter<Self, fn(&Pos) -> bool>
    where
        Self: Sized;
}

impl<I: Iterator<Item = Pos>> IteratorExt for I {
    fn on_board(self) -> Filter<Self, fn(&Pos) -> bool> {
        self.filter(|pos| Board::on_board(*pos))
    }
}

mod sealed {
    use crate::pos::Pos;

    pub trait Sealed {}

    impl<I: Iterator<Item = Pos>> Sealed for I {}
}
