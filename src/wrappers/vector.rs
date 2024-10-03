use crate::{impls, RandomVectorTrait, SequentialVectorTrait};

#[derive(Default, Clone)]
pub struct SequentialVector<V: SequentialVectorTrait> {
    pub(crate) object: V,
}

impl<V: SequentialVectorTrait> std::ops::Deref for SequentialVector<V> {
    type Target = V;
    fn deref(&self) -> &Self::Target {
        &self.object
    }
}

impl<V: SequentialVectorTrait> std::ops::DerefMut for SequentialVector<V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.object
    }
}

impl<V: SequentialVectorTrait> std::fmt::Debug for SequentialVector<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        debug_vector(f, &self.object)
    }
}

#[derive(Default, Clone)]
pub struct RandomVector<V: RandomVectorTrait> {
    pub(crate) object: V,
}

impl<V: RandomVectorTrait> std::ops::Deref for RandomVector<V> {
    type Target = V;
    fn deref(&self) -> &Self::Target {
        &self.object
    }
}

impl<V: RandomVectorTrait> std::ops::DerefMut for RandomVector<V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.object
    }
}

impl<V: RandomVectorTrait> std::fmt::Debug for RandomVector<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        debug_vector(f, &self.object)
    }
}

/// 次元と同サイズの配列と等価な密ベクトル
pub type DenseVector = RandomVector<impls::DenseVector>;

impl DenseVector {
    pub fn new<I: Iterator<Item = (usize, f64)>>(dimension: usize, nonzero_elements: I) -> Self {
        Self { object: impls::DenseVector::new(dimension, nonzero_elements) }
    }
}

pub type SparseVector = RandomVector<impls::SparseVector>;

impl SparseVector {
    pub fn new<I: Iterator<Item = (usize, f64)>>(dimension: usize, nonzero_elements: I) -> Self {
        Self { object: impls::SparseVector::new(dimension, nonzero_elements) }
    }
}

/// 非ゼロ要素と同サイズの配列と等価な疎ベクトル
pub type CompressedVector = SequentialVector<impls::CompressedVector>;

impl CompressedVector {
    pub fn new<I: Iterator<Item = (usize, f64)>>(dimension: usize, nonzero_elements: I) -> Self {
        Self { object: impls::CompressedVector::new(dimension, nonzero_elements) }
    }
}

fn debug_vector(f: &mut std::fmt::Formatter, vector: &impl SequentialVectorTrait) -> std::fmt::Result {
    write!(f, "{{ dimension = {}, values = [", vector.dimension())?;
    let mut first = true;
    for (index, value) in vector.iter() {
        if first {
            first = false;
        } else {
            write!(f, ", ")?;
        }
        write!(f, "({}, {})", index, value)?;
    }
    write!(f, "] }}")?;
    return Ok(());
}
