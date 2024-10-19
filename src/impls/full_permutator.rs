use crate::traits::{MutPermutatorTrait, PermutatorTrait};
use crate::types::{COLUMN, NULL_INDEX, ROW};

#[derive(Default, Clone, Debug)]
pub struct FullPermutator {
    permutations: Vec<usize>,
    unpermutations: Vec<usize>,
}

impl FullPermutator {
    // pub fn new(dimension: [usize; 2]) -> Self {
    //     let k = *dimension.iter().min().unwrap();
    //     Self {
    //         permutations: Vec::from_iter((0..k).chain(std::iter::repeat_n(NULL_INDEX, dimension[COLUMN] - k))),
    //         unpermutations: Vec::from_iter((0..k).chain(std::iter::repeat_n(NULL_INDEX, dimension[ROW] - k))),
    //     }
    // }

    #[inline(always)]
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
    #[inline(always)]
    fn dimension(&self) -> [usize; 2] {
        [self.unpermutations.len(), self.permutations.len()]
    }

    #[inline(always)]
    fn permutate(&self, i: usize) -> Option<usize> {
        let j = self.permutations[i];
        if j != NULL_INDEX {
            return Some(j);
        } else {
            return None;
        }
    }

    #[inline(always)]
    fn unpermutate(&self, i: usize) -> Option<usize> {
        let j = self.unpermutations[i];
        if j != NULL_INDEX {
            return Some(j);
        } else {
            return None;
        }
    }

    #[inline(always)]
    fn iter(&self) -> impl Iterator<Item = (usize, usize)> + Clone + '_ {
        self.permutations.iter().cloned().enumerate().filter(|(_, to)| *to != NULL_INDEX)
    }
}

impl MutPermutatorTrait for FullPermutator {
    fn replace_by_iter<I: Iterator<Item = (usize, usize)>>(&mut self, dimension: [usize; 2], permutations: I) {
        self.permutations.clear();
        self.unpermutations.clear();
        let k = *dimension.iter().min().unwrap();
        self.permutations.extend((0..k).chain(std::iter::repeat_n(NULL_INDEX, dimension[COLUMN] - k)));
        self.unpermutations.extend((0..k).chain(std::iter::repeat_n(NULL_INDEX, dimension[ROW] - k)));
        for (from, to) in permutations {
            self.set(from, to);
        }
    }
}
