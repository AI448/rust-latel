use crate::traits::{MatrixTrait, PermutatorTrait, SequentialMatrixTrait};
use crate::types::{Direction, Direction::COLUMN, Direction::ROW};

#[derive(Debug)]
pub struct RowPermutatedMatrix<P: PermutatorTrait, M: MatrixTrait> {
    permutator: P,
    matrix: M,
}

impl<P: PermutatorTrait, M: MatrixTrait> RowPermutatedMatrix<P, M> {
    pub fn new(permutator: P, matrix: M) -> Self {
        assert!(permutator.dimension::<{ COLUMN }>() == matrix.dimension::<{ ROW }>());
        Self { permutator: permutator, matrix: matrix }
    }
}

impl<P: PermutatorTrait, M: MatrixTrait> MatrixTrait for RowPermutatedMatrix<P, M> {
    fn dimension<const D: Direction>(&self) -> usize {
        match D {
            ROW => self.matrix.dimension::<{ ROW }>(),
            COLUMN => self.matrix.dimension::<{ COLUMN }>(),
        }
    }
}

impl<P: PermutatorTrait, M: SequentialMatrixTrait> SequentialMatrixTrait for RowPermutatedMatrix<P, M> {
    fn iter(&self) -> impl Iterator<Item = ([usize; 2], f64)> + Clone + '_ {
        self.matrix.iter().filter_map(|([i1, j], x)| self.permutator.permutate(i1).map(|i2| ([i2, j], x)))
    }
}
