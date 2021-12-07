// 数値を画面に表示するマクロを定義 --- (*1)
macro_rules! echo_num {
    ($num:expr) => { println!("{}", $num); }
}

// マクロを利用する例 --- (*2)
fn main() {
    echo_num!(10);
    echo_num![20];
    echo_num!{30}
}

