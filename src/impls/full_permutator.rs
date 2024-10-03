use crate::traits::PermutatorTrait;
use crate::types::{
    Direction::{self, COLUMN, ROW},
    NULL_INDEX,
};

#[derive(Default, Clone, Debug)]
pub struct FullPermutator {
    permutations: Vec<usize>,
    unpermutations: Vec<usize>,
}

impl FullPermutator {
    pub fn new(dimension: [usize; 2]) -> Self {
        let k = *dimension.iter().min().unwrap();
        Self {
            permutations: Vec::from_iter((0..k).chain(std::iter::repeat_n(NULL_INDEX, dimension[COLUMN] - k))),
            unpermutations: Vec::from_iter((0..k).chain(std::iter::repeat_n(NULL_INDEX, dimension[ROW] - k))),
        }
    }

    pub fn set(&mut self, from: usize, to: usize) {
        let i = self.permutations[from];
        let j = self.unpermutations[to];
        self.permutations[from] = to;
        self.unpermutations[to] = from;
        if j != NULL_INDEX {
            self.permutations[j] = i;
        }
        if i != NULL_INDEX {
            self.unpermutations[i] = j;
        }
    }
}

impl PermutatorTrait for FullPermutator {
    fn dimension<const D: Direction>(&self) -> usize {
        match D {
            ROW => self.unpermutations.len(),
            COLUMN => self.permutations.len(),
        }
    }

    fn permutate(&self, i: usize) -> Option<usize> {
        let j = self.permutations[i];
        if j != NULL_INDEX {
            return Some(j);
        } else {
            return None;
        }
    }
    fn unpermutate(&self, i: usize) -> Option<usize> {
        let j = self.unpermutations[i];
        if j != NULL_INDEX {
            return Some(j);
        } else {
            return None;
        }
    }
}
