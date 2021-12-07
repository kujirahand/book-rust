#[cfg(test)]
mod tests {
    #[test]
    fn calc_test1() {
        // 簡単な計算のテストその1
        assert_eq!(100 * 2, 200);
        assert_eq!((1 + 2) * 3, 9);
        assert_eq!(1 + 2 * 3, 7);
    }
    #[test]
    fn calc_test2() {
        // 簡単な計算のテストその2
        assert_eq!(2 * 3, 6);
        // わざとテストに失敗してみる
        assert_eq!(2 * 3, 7);
    }
}
