// 複数の数値を表示するマクロを定義 --- (*1)
#[macro_export]
macro_rules! echo_nums {
    ( $( $num:expr ),* ) => {
        $(
            print!("{}, ", $num);
        )*
        println!("");
    }
}

// マクロを利用する --- (*2)
fn main() {
    echo_nums![10, 20, 30, 40, 50];
}

