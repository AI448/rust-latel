use fxhash::FxHashMap as HashMap;

use crate::{
    traits::{MatrixTrait, RowMatrixTrait, SequentialMatrixTrait, SequentialVectorTrait},
    types::Direction::{self, COLUMN, ROW},
};

use super::VectorView;

#[derive(Clone, Debug)]
pub struct CRSMatrix {
    column_dimension: usize,
    row_positions: Vec<usize>,
    column_indices: Vec<usize>,
    values: Vec<f64>,
}

impl CRSMatrix {
    pub fn new<I: Iterator<Item = ([usize; 2], f64)>>(dimension: [usize; 2], nonzero_elements: I) -> Self {
        let mut buffer = HashMap::default();
        for ([i, j], x) in nonzero_elements {
            assert!(i < dimension[ROW]);
            assert!(j < dimension[COLUMN]);
            if x != 0.0 {
                buffer.insert([i, j], x);
            }
        }

        let mut row_positions = Vec::default();
        row_positions.resize(dimension[ROW] + 1, 0);
        let mut column_indices = Vec::default();
        column_indices.resize(buffer.len(), 0);
        let mut values = Vec::default();
        values.resize(buffer.len(), 0.0);

        // 各行の要素数をカウント
        for ([i, _], _) in buffer.iter() {
            row_positions[*i] += 1;
        }
        // 各行の終端位置を特定
        for i in 0..dimension[ROW] {
            row_positions[i + 1] += row_positions[i];
        }
        // 要素を追加
        for ([i, j], x) in buffer.iter() {
            let p = &mut row_positions[*i];
            *p -= 1;
            column_indices[*p] = *j;
            values[*p] = *x;
        }
        debug_assert!(row_positions[0] == 0);

        Self {
            column_dimension: dimension[COLUMN],
            row_positions: row_positions,
            column_indices: column_indices,
            values: values,
        }
    }
}

impl MatrixTrait for CRSMatrix {
    fn dimension<const D: Direction>(&self) -> usize {
        match D {
            Direction::ROW => self.row_positions.len() - 1,
            Direction::COLUMN => self.column_dimension,
        }
    }
}

impl SequentialMatrixTrait for CRSMatrix {
    fn iter(&self) -> impl Iterator<Item = ([usize; 2], f64)> + Clone + '_ {
        (0..self.dimension::<{ Direction::ROW }>()).flat_map(move |i| {
            let from = self.row_positions[i as usize];
            let to = self.row_positions[i as usize + 1];
            (from..to).map(move |k| ([i, self.column_indices[k]], self.values[k]))
        })
    }
}

impl RowMatrixTrait for CRSMatrix {
    fn row(&self, i: usize) -> impl SequentialVectorTrait + '_ {
        let from = self.row_positions[i as usize];
        let to = self.row_positions[i as usize + 1];
        VectorView::new(self.column_dimension, (from..to).map(|k| (self.column_indices[k], self.values[k])))
    }
}
