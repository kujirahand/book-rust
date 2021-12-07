use std::rc::Rc;
use std::cell::RefCell;
fn main() {
    // ヒープに内部可変性を持ったi32型を確保 --- (*1)
    let a = Rc::new(RefCell::new(1000));
    // 参照カウンタを加算 --- (*2)
    let b = Rc::clone(&a);
    // 値を変更する --- (*3)
    *b.borrow_mut() += 100;
    // 値が変更された --- (*4)
    println!("{}", a.borrow());
}

