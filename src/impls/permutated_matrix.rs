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
            ROW => self.permutator.dimension::<{ ROW }>(),
            COLUMN => self.matrix.dimension::<{ COLUMN }>(),
        }
    }
}

impl<P: PermutatorTrait, M: SequentialMatrixTrait> SequentialMatrixTrait for RowPermutatedMatrix<P, M> {
    fn iter(&self) -> impl Iterator<Item = ([usize; 2], f64)> + Clone + '_ {
        self.matrix.iter().filter_map(|([i1, j], x)| self.permutator.permutate(i1).map(|i2| ([i2, j], x)))
    }
}

#[derive(Debug)]
pub struct ColumnPermutatedMatrix<M: MatrixTrait, P: PermutatorTrait> {
    matrix: M,
    permutator: P,
}

impl<M: MatrixTrait, P: PermutatorTrait> ColumnPermutatedMatrix<M, P> {
    pub fn new(matrix: M, permutator: P) -> Self {
        assert!(matrix.dimension::<{ COLUMN }>() == permutator.dimension::<{ ROW }>());
        Self { matrix: matrix, permutator: permutator }
    }
}

impl<M: MatrixTrait, P: PermutatorTrait> MatrixTrait for ColumnPermutatedMatrix<M, P> {
    fn dimension<const D: Direction>(&self) -> usize {
        match D {
            ROW => self.matrix.dimension::<{ ROW }>(),
            COLUMN => self.permutator.dimension::<{ COLUMN }>(),
        }
    }
}

impl<M: SequentialMatrixTrait, P: PermutatorTrait> SequentialMatrixTrait for ColumnPermutatedMatrix<M, P> {
    fn iter(&self) -> impl Iterator<Item = ([usize; 2], f64)> + Clone + '_ {
        self.matrix.iter().filter_map(|([i, j1], x)| self.permutator.unpermutate(j1).map(|j2| ([i, j2], x)))
    }
}
