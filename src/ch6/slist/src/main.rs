mod slist;
fn main() {
    // リストを作成
    let mut list = slist::List::new();
    // 値を末尾に追加
    list.push(100);
    list.push(200);
    // 先頭に値を追加
    list.unshift(10);
    list.unshift(20);
    // 任意のインデックスを取得
    println!("{}", list.get(0).unwrap());
    println!("{}", list.get(1).unwrap());
    println!("{}", list.get(2).unwrap());
    println!("{}", list.get(3).unwrap());
}
