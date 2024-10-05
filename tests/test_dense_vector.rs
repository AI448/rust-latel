use latel::DenseVector;

#[test]
fn test() {
    let x = DenseVector::new(3, [(0, 0.0), (1, 1.0), (2, 2.0)].into_iter());
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
    // clone して
    let mut a = x.clone();
    // 足してみたり
    a += &x;
    dbg!(&a);
    // 引いてみたり
    a -= &z;
    dbg!(&a);
    // クリアしてみたり
    a.zero_clear();
    dbg!(&a);
    // // 代入してみたり
    // a <<= &x;
    // dbg!(&a);
}
