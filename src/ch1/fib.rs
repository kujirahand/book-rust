// フィボナッチ数列を求める
fn main() {
    let mut a = 1;
    let mut b = 1;
    println!("{}", a);
    println!("{}", b);
    for _ in 0..30 {
        println!("{}", a+b);
        let tmp = a;
        a = b;
        b = tmp + b;
    }
}

