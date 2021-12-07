use std::rc::Rc;
fn main() {
    // ヒープにi32型の値を確保 --- (*1)
    let a_rc = Rc::new(1000);
    {
        // i32型への参照をb_rcにも作成 --- (*2)
        let b_rc = Rc::clone(&a_rc);
        println!("{}", b_rc);
        // a_rcの参照カウンタを確認 --- (*3)
        println!("参照数={}", Rc::strong_count(&a_rc));
    } // ここでa_rcの参照カウンタが-1される
    println!("{}", a_rc); // Rc型なので利用可能 --- (*4)
    println!("参照数={}", Rc::strong_count(&a_rc));
}

