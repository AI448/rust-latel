use latel::{DenseVector, FullPermutator, SequentialVectorTrait};

#[test]
fn test() {
    let mut p = FullPermutator::new([3, 3]);
    p.set(0, 1);
    let v = DenseVector::new(3, [(1, 1.0), (2, 2.0)].into_iter());
    dbg!(&v);
    // 置換してついでにスカラー倍してみる
    let x = 10.0 * (&p * &v);
    dbg!(&x);
    // 転置で置換してもとに戻してみる
    let y = p.T() * &x / 10.0;
    dbg!(&y);

    // クローンしてみて
    let mut z = v.clone();
    // y を引くと
    z -= y;
    // 0 になっているはず
    dbg!(z.norm());
    assert!(z.norm() <= 1e-10);
}
