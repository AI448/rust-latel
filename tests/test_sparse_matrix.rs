use latel::{FullPermutator, SparseMatrix};

#[test]
fn test() {
    let mut x = SparseMatrix::generate_from_iter([3, 3], [([0, 0], 1.0), ([1, 1], 1.1), ([2, 2], 2.2)].into_iter());
    dbg!(&x);
    x[[0, 0]] = 0.0;
    dbg!(&x[[0, 0]]);
    x[[1, 2]] = 1.2;
    dbg!(&x[[1, 2]]);
    dbg!(&x);
    {
        let mut p = FullPermutator::generate_from_iter([3, 3], [].into_iter());
        p.set(0, 1);
        // 置換行列を掛けてみる
        let y = &p * &x;
        dbg!(&y);
        // 右から掛けてみる
        let z = &x * &p;
        dbg!(&z);
    }
    // 行を 0 クリア
    x.clear_row(1);
    dbg!(&x);
    // 要素を削除
    x.remove([0, 0]);
    dbg!(&x);
}
