use std::rc::{Rc,Weak};
use std::cell::RefCell;
// 双方向リストの定義
struct Node {
    data: isize,
    prev: Option<Weak<RefCell<Node>>>, // 弱い参照
    next: Option<Rc<RefCell<Node>>>, // 強い参照
}
fn main() {
    // 値を生成
    let a = Rc::new(RefCell::new(
            Node{data:10, prev:None, next:None}));
    let b = Rc::new(RefCell::new(
            Node{data:20, prev:None, next:None}));
    // aとbを相互に参照
    a.borrow_mut().next = Some(Rc::clone(&b));
    b.borrow_mut().prev = Some(Rc::downgrade(&a));
    // 参照カウントを確認
    println!("a: {}", Rc::strong_count(&a));
    println!("b: {}", Rc::strong_count(&b));
    // 値を表示
    println!("b.data= {}", b.borrow().data);
    // bからaの値を得る
    match &b.borrow().prev {
        None => {},
        Some(prev) => {
            // 
            let pa = prev.upgrade().unwrap();
            println!("a.data= {}", pa.borrow().data);
        },
    };
}

