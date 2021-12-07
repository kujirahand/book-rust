// 問題のあるプログラム
use std::rc::Rc;
fn main() {
    // ヒープにi32型の値を確保
    let mut a_rc = Rc::new(1000);
    // a_rcの参照を複製 --- (*1)
    let mut b_rc = Rc::clone(&a_rc);
    // ここで値を変更したいが...
    *b_rc += 100; // 変更できない --- (*2)
    println!("{}", b_rc);
}

