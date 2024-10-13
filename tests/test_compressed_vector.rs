use latel::CompressedVector;

#[test]
fn test() {
    let mut x = CompressedVector::from_iter(3, [(0, 0.0), (1, 1.0), (2, 2.0)].into_iter());
    dbg!(&x);
    {
        // 掛けたり
        let y = 10.0 * &x;
        dbg!(&y);
        // 割ったり
        let z = -&y / 10.0;
        dbg!(&z);
    }
    // クリアしてみたり
    x.clear();
    dbg!(&x);
}
