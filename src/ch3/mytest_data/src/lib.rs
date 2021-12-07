#[cfg(test)]
mod tests {
    #[test]
    fn array_test() {
        // 数値配列を初期化 --- (*1)
        let a1 = [100, 200, 300];
        let a2 = [100, 200, 300];
        assert_eq!(a1, a2);
        // Stringの配列を初期化 --- (*2)
        let a3 = [
            "リンゴ".to_string(), 
            "バナナ".to_string()];
        let a4 = [
            String::from("リンゴ"), 
            String::from("バナナ")];
        assert_eq!(a3, a4);
    }
    #[test]
    fn vec_test() {
        // ベクタ(Vec<str>)を初期化 --- (*3)
        let v1 = vec!["apple", "banana", "mango"];
        let mut v2:Vec<&str> = Vec::new();
        v2.push("apple");
        v2.push("banana");
        v2.push("mango");
        assert_eq!(v1, v2);
    }
}
