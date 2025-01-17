use latel::{DenseVector, FullPermutator, GenerateFrom, SequentialVectorTrait};

#[test]
fn test() {
    let mut p = FullPermutator::generate_from_iter([3, 3], [].into_iter());
    p.set(0, 1);
    let v = DenseVector::generate_from_iter(3, [(1, 1.0), (2, 2.0)].into_iter());
    dbg!(&v);
    // 置換してついでにスカラー倍してみる
    let x = 10.0 * (&p * &v);
    dbg!(&x);
    // 転置で置換してもとに戻してみる
    let y = p.T() * &x / 10.0;
    dbg!(&y);

    // クローンしてみて
    let mut z = DenseVector::generate_from(&v);
    // y を引くと
    z -= y;
    // 0 になっているはず
    dbg!(z.norm());
    assert!(z.norm() <= 1e-10);
}
