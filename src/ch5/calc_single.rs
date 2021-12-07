use std::time::Instant;
fn main() {
    // 求めたいフィボナッチ数の一覧
    let request_nums = [43, 42, 20, 39, 37, 35, 30];
    let start_time = Instant::now();
    // スレッドなしで計算
    for num in request_nums {
        let answer = fib(num);
        println!("[結果] fib({})={}", num, answer);
    }
    show_time(start_time);
}
// 再帰的にフィボナッチ数を調べる関数
fn fib(n: i64) -> i64 {
    if n == 1 { return 0; }
    if n == 2 { return 1; }
    return fib(n - 2) + fib(n - 1);
}
fn show_time(start_time: Instant) {
    let elapsed = start_time.elapsed();
    println!("実行時間: {:?}", elapsed);
}

