// 単方向リスト --- (*1)
pub struct Node {
    data: i64,
    link: Option<Box<Node>>
}
// 手軽に単方向リストを生成する関数 --- (*2)
fn node(v: i64, link: Option<Box<Node>>) -> Option<Box<Node>> {
    Some(Box::new(Node{data:v, link:link}))
}
fn main() {
    // 単方向リストを生成 --- (*3)
    // セルを生成
    let mut c1 = node(10, None).unwrap();
    let mut c2 = node(20, None).unwrap();
    let mut c3 = node(30, None).unwrap();
    // 単方向リストとして接続する
    c2.link = Some(c3);
    c1.link = Some(c2);
    let c = c1;
    // 各要素を取り出して表示 --- (*5)
    let mut p = &c;
    loop {
        println!("{}", p.data);
        // pが次の要素を指すように変更
        match p.link {
            None => break,
            Some(ref link) => p = &link,
        }
    }
}

