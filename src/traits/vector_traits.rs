/// ベクトル
pub trait VectorTrait {
    /// 次元
    fn dimension(&self) -> usize;
}

// NOTE: into_iter(self) が可能なベクトルがあってもいいのかも

pub trait LazyVectorTrait: VectorTrait {
    fn add_assign_to(&self, vector: &mut impl RandomMutVectorTrait);
}

/// シーケンシャルアクセス可能なベクトル
pub trait SequentialVectorTrait: VectorTrait {
    /// 非ゼロ要素の添字と値の組のイテレータ
    fn iter(&self) -> impl Iterator<Item = (usize, f64)> + Clone + '_;

    fn norm(&self) -> f64 {
        // NOTE: 後で考える fma を使ったほうが高精度だったりするのだろうか
        self.iter().map(|(_, x)| x.powi(2)).sum::<f64>().sqrt()
    }
    // TODO: 後で検討
    // fn add_assign_to(&self, lhs: &mut RandomMutVectorTrait)
    // のような関数を定義して，実装を右辺値に応じて切り替えることができる
    // それによって，右辺値が スカラー×ベクトル であれば fma 命令を使うなどの特殊化っぽいことができるようになる
}

/// ランダムアクセス可能なベクトル
pub trait RandomVectorTrait: SequentialVectorTrait {
    fn get(&self, i: usize) -> f64;
}

/// 代入可能なベクトル
pub trait SequentialMutVectorTrait: SequentialVectorTrait {
    fn replace<I: Iterator<Item = (usize, f64)>>(&mut self, dimension: usize, nonzero_elements: I);
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
    fn dimension(&self) -> usize {
        (*self).dimension()
    }
}

impl<V: SequentialVectorTrait> SequentialVectorTrait for &V {
    fn iter(&self) -> impl Iterator<Item = (usize, f64)> + Clone + '_ {
        (*self).iter()
    }
}

impl<V: RandomVectorTrait> RandomVectorTrait for &V {
    fn get(&self, i: usize) -> f64 {
        (*self).get(i)
    }
}
