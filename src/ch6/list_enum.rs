// 列挙型でNodeを定義 --- (*1)
enum Node {
    Empty,
    Cons(i64, Box<Node>),
}
// 列挙型を手軽に使えるよう宣言 --- (*2)
use Node::{Empty, Cons};
fn node(v: i64, link: Box<Node>) -> Box<Node> {
    Box::new(Cons(v, link))
}
fn main() {
    // 単方向リストを生成 --- (*3)
    let c = node(10, node(20, node(30, Box::new(Empty))));

    // 先頭から各要素をたどって表示 --- (*4)
    let mut ptr: &Box<Node> = &c;
    loop {
        // &Box<Node>からNodeを取り出す --- (*5)
        let cur_node: &Node = &**ptr; 
        match cur_node {
            Empty => break,
            Cons(v, link) => {
                println!("{}", v);
                ptr = &link;
            }
        }
    }
}


