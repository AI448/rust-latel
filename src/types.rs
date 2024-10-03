use std::marker::ConstParamTy;

#[derive(Clone, Copy, PartialEq, Eq, ConstParamTy)]
pub enum Direction {
    ROW = 0,
    COLUMN = 1,
}

impl std::ops::Not for Direction {
    type Output = Direction;
    fn not(self) -> Self::Output {
        match self {
            Direction::ROW => Direction::COLUMN,
            Direction::COLUMN => Direction::ROW,
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

pub const NULL_INDEX: usize = usize::MAX;
