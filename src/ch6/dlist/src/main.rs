mod dlist;
fn main() {
    // リストに値を追加
    let mut list = dlist::List::new();
    // 末尾に値を追加
    list.push(100);
    list.push(110);
    // 先頭に値を追加
    list.unshift(10);
    list.unshift(20);
    // イテレータで値を全部表示
    for v in list.iter() {
        println!("{}", v);
    }
}
