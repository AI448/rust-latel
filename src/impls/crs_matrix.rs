use fxhash::FxHashMap as HashMap;

use crate::{
    traits::{MatrixTrait, RowMatrixTrait, SequentialMatrixTrait, SequentialMutMatrixTrait},
    types::{COLUMN, ROW},
};

#[derive(Default, Clone, Debug)]
pub struct CRSMatrix {
    column_dimension: usize,
    row_positions: Vec<usize>,
    column_indices: Vec<usize>,
    values: Vec<f64>,
}

impl SequentialMutMatrixTrait for CRSMatrix {
    fn replace_by_iter<I: Iterator<Item = ([usize; 2], f64)>>(&mut self, dimension: [usize; 2], nonzero_elements: I) {
        let mut buffer = HashMap::default();
        for ([i, j], x) in nonzero_elements {
            assert!(i < dimension[ROW]);
            assert!(j < dimension[COLUMN]);
            assert!(!buffer.contains_key(&[i, j]));
            if x != 0.0 {
                buffer.insert([i, j], x);
            }
        }

        self.column_dimension = dimension[COLUMN];
        self.row_positions.clear();
        self.row_positions.resize(dimension[ROW] + 1, 0);
        self.column_indices.clear();
        self.column_indices.resize(buffer.len(), 0);
        self.values.clear();
        self.values.resize(buffer.len(), 0.0);

        // 各行の要素数をカウント
        for ([i, _], _) in buffer.iter() {
            self.row_positions[*i] += 1;
        }
        // 各行の終端位置を特定
        for i in 0..dimension[ROW] {
            self.row_positions[i + 1] += self.row_positions[i];
        }
        // 要素を追加
        for ([i, j], x) in buffer.iter() {
            let p = &mut self.row_positions[*i];
            *p -= 1;
            self.column_indices[*p] = *j;
            self.values[*p] = *x;
        }
        debug_assert!(self.row_positions[0] == 0);
    }
}

impl MatrixTrait for CRSMatrix {
    #[inline(always)]
    fn dimension(&self) -> [usize; 2] {
        [if self.row_positions.len() == 0 { 0 } else { self.row_positions.len() - 1 }, self.column_dimension]
    }
}

impl SequentialMatrixTrait for CRSMatrix {
    #[inline(always)]
    fn iter(&self) -> impl Iterator<Item = ([usize; 2], f64)> + Clone + '_ {
        (0..self.dimension()[ROW]).flat_map(move |i| {
            let from = self.row_positions[i as usize];
            let to = self.row_positions[i as usize + 1];
            (from..to).map(move |k| ([i, self.column_indices[k]], self.values[k]))
        })
    }
}

impl RowMatrixTrait for CRSMatrix {
    #[inline(always)]
    fn iter_row(&self, i: usize) -> impl Iterator<Item=(usize, f64)> + Clone + '_ {
        let from = self.row_positions[i as usize];
        let to = self.row_positions[i as usize + 1];
        (from..to).map(|k| (self.column_indices[k], self.values[k]))
    }
}
