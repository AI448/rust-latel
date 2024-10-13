/// ベクトル
pub trait VectorTrait {
    /// 次元
    fn dimension(&self) -> usize;

    /// RandomMutVectorTrait への代入
    fn assign_to_random_vector(&self, lhs: &mut impl RandomMutVectorTrait);

    /// RandomMutVectorTrait への加算
    fn add_to_random_vector(&self, lhs: &mut impl RandomMutVectorTrait);

    /// RandomMutVectorTrait からの減算
    fn sub_from_random_vector(&self, lhs: &mut impl RandomMutVectorTrait);
}

// NOTE: into_iter(self) が可能なベクトルがあってもいいのかも

/// シーケンシャルアクセス可能なベクトル
pub trait SequentialVectorTrait: VectorTrait {
    /// 非ゼロ要素の添字と値の組のイテレータ
    fn iter(&self) -> impl Iterator<Item = (usize, f64)> + Clone + '_;

    #[inline(always)]
    fn norm(&self) -> f64 {
        // NOTE: 後で考える fma を使ったほうが高精度だったりするのだろうか
        self.iter().map(|(_, x)| x.powi(2)).sum::<f64>().sqrt()
    }
}

/// ランダムアクセス可能なベクトル
pub trait RandomVectorTrait: SequentialVectorTrait {
    fn get(&self, i: usize) -> f64;
}

/// 代入可能なベクトル
pub trait SequentialMutVectorTrait: SequentialVectorTrait + Default {
    fn generate_from_iter<I: Iterator<Item = (usize, f64)>>(dimension: usize, nonzero_elements: I) -> Self {
        let mut vector = Self::default();
        vector.replace_by_iter(dimension, nonzero_elements);
        return vector;
    }

    fn replace_by_iter<I: Iterator<Item = (usize, f64)>>(&mut self, dimension: usize, nonzero_elements: I);
}

/// ランダムアクセスによる変更が可能なベクトル
pub trait RandomMutVectorTrait:
    RandomVectorTrait
    + SequentialMutVectorTrait
    + std::ops::Index<usize, Output = f64>
    + std::ops::IndexMut<usize, Output = f64>
{
}

// VectorTrait への参照も VectorTrait とする

impl<V: VectorTrait> VectorTrait for &V {
    #[inline(always)]
    fn dimension(&self) -> usize {
        (*self).dimension()
    }
    #[inline(always)]
    fn assign_to_random_vector(&self, lhs: &mut impl RandomMutVectorTrait) {
        (*self).assign_to_random_vector(lhs);
    }
    #[inline(always)]
    fn add_to_random_vector(&self, lhs: &mut impl RandomMutVectorTrait) {
        (*self).add_to_random_vector(lhs);
    }
    #[inline(always)]
    fn sub_from_random_vector(&self, lhs: &mut impl RandomMutVectorTrait) {
        (*self).sub_from_random_vector(lhs);
    }
}

impl<V: SequentialVectorTrait> SequentialVectorTrait for &V {
    #[inline(always)]
    fn iter(&self) -> impl Iterator<Item = (usize, f64)> + Clone + '_ {
        (*self).iter()
    }
}

impl<V: RandomVectorTrait> RandomVectorTrait for &V {
    #[inline(always)]
    fn get(&self, i: usize) -> f64 {
        (*self).get(i)
    }
}
