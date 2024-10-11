use std::marker::ConstParamTy;

#[derive(Clone, Copy, PartialEq, Eq, ConstParamTy)]
pub enum Direction {
    ROW = 0,
    COLUMN = 1,
}

pub use Direction::{COLUMN, ROW};

impl std::ops::Not for Direction {
    type Output = Direction;
    fn not(self) -> Self::Output {
        match self {
            ROW => COLUMN,
            COLUMN => ROW,
        }
    }
}

impl<T> std::ops::Index<Direction> for [T; 2] {
    type Output = T;
    fn index(&self, direction: Direction) -> &Self::Output {
        &self[direction as usize]
    }
}

impl<T> std::ops::IndexMut<Direction> for [T; 2] {
    fn index_mut(&mut self, direction: Direction) -> &mut Self::Output {
        &mut self[direction as usize]
    }
}

pub trait Transposed {
    type Output;
    fn transposed(&self) -> Self::Output;
}

impl<T: Clone> Transposed for [T; 2] {
    type Output = [T; 2];
    fn transposed(&self) -> Self::Output {
        [self[1].clone(), self[0].clone()]
    }
}

pub const NULL_INDEX: usize = usize::MAX;

pub const ZERO: f64 = 0.0;
