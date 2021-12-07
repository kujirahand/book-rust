// 再帰的に呼び出す関数sumを定義
fn sum(n:i32) -> i32 {
    if n <= 0 { return 0; } // 再帰の終了条件 --- (*1)
    return sum(n-1) + n; // 再帰的に呼び出す --- (*2)
}

fn main() {
    println!("{}", sum(10));
}

