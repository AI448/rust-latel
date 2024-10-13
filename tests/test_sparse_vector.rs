use latel::{GenerateFrom, SequentialVectorTrait, SparseVector};

#[test]
fn test() {
    let x = SparseVector::from_iter(3, [(0, 0.0), (1, 1.0), (2, 2.0)].into_iter());
    assert!(x[0] == 0.0);
    assert!(x[1] == 1.0);
    assert!(x[2] == 2.0);

    dbg!(&x);
    // 掛けたり
    let y = 10.0 * &x;
    dbg!(&y);
    // 割ったり
    let z = -&y / 10.0;
    dbg!(&z);
    // 内積をとってみたり
    let n = &x * &z;
    dbg!(n);
    // クローンして
    let mut x = SparseVector::generate_from(&x);
    dbg!(&x);
    // 足してみたり
    x += &z;
    dbg!(&x);
    assert!(x.norm() <= 1e-10);
}
