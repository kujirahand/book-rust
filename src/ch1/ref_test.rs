fn main() {
    // 変数 v を 10 にする --- (*1)
    let mut v = 10;

    // 関数を呼び出す --- (*2)
    set_value(&mut v);
    
    // 変数 value の値は？ --- (*3)
    println!("v={}", v);
}

// 引数の値を100に変更する関数 --- (*4)
fn set_value(arg: &mut u32) {
    *arg = 100;
}

