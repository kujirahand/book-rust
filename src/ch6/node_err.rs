use std::rc::Rc;
use std::cell::RefCell;
// 双方向リストの定義(循環参照の問題あり)
struct Node {
    data: isize,
    prev: Option<Rc<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
}

fn main() {
    // 値を生成
    let a = Rc::new(RefCell::new(
            Node{data:10, prev:None, next:None}));
    let b = Rc::new(RefCell::new(
            Node{data:20, prev:None, next:None}));
    // aとbを相互に参照
    b.borrow_mut().prev = Some(Rc::clone(&a));
    a.borrow_mut().next = Some(Rc::clone(&b));
    // 参照カウントを確認
    println!("a: {}", Rc::strong_count(&a));
    println!("b: {}", Rc::strong_count(&b));
    println!("{}", a.borrow().data);
}

