use latel::{FullPermutator, SparseMatrix};

#[test]
fn test() {
    let mut x = SparseMatrix::new([3, 3], [([0, 0], 1.0), ([1, 1], 1.1), ([2, 2], 2.2)].into_iter());
    dbg!(&x);
    x[[0, 0]] = 0.0;
    x[[1, 2]] = 1.2;
    dbg!(&x);

    {
        let mut p = FullPermutator::new([3, 3]);
        p.set(0, 1);
        // 置換行列を掛けてみる
        let y = &p * &x;
        dbg!(&y);
    }
    // 行を 0 クリア
    x.clear_row(1);
    dbg!(&x);
}