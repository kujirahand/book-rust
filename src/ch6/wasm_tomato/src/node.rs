// 文法要素をNode型として定義 --- (*1)
#[derive(Debug, Clone)]
pub enum Node {
    Nop, // 何もしない
    Number(i32), // 数値を表す
    Calc(char, Box<Node>, Box<Node>), // 計算式
    If(Box<Node>, Box<Vec<Node>>, Box<Vec<Node>>), // if文
    For(String, i32, i32, Box<Vec<Node>>), // for文
    Print(Box<Node>), // print文(計算出力)
    PrintStr(String), // print文(定数出力)
    SetVar(String, Box<Node>), // 変数代入
    GetVar(String), // 変数参照
}
impl Node {
    // 手軽にNode::Calc型を返す関数 --- (*2)
    pub fn calc(op: char, l: Node, r: Node) -> Node {
        Node::Calc(op, Box::new(l), Box::new(r))
    }
    // 手軽にNode::If型を返す関数 --- (*3)
    pub fn if_(cond: Node, t: Vec<Node>, f: Vec<Node>) -> Node {
        Node::If(Box::new(cond), Box::new(t), Box::new(f))
    }
}
