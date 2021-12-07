// 商品を表す構造体を定義 --- (*1)
#[derive(Debug,PartialEq)]
struct GItem {
    name: String,
    price: i64,
}

#[cfg(test)]
mod tests {
    use super::*; // 外側の要素を利用 --- (*2)
    #[test]
    fn item_test() {
        // 構造体を初期化 --- (*3)
        let apple1 = GItem{
            name: String::from("リンゴ"),
            price: 2400,
        };
        let mut apple2 = GItem{
            name: "リンゴ".to_string(),
            price: 0,
        };
        apple2.price = 2400;
        
        // 構造体のフィールドを比較 --- (*4)
        assert_eq!(apple1.name, apple2.name);
        assert_eq!(apple1.price, apple2.price);
        
        // 構造体全体を直接比較 --- (*5)
        assert_eq!(apple1, apple2);
    }
}
