// 単方向リストを実装したいがエラーになる
pub struct Node {
    data: i64,
    link: Option<Node>
}
fn main() {
    let mut c = Node{data: 30, link: None};
    println!("{}", c.data);
}

