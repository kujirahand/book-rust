// RustでFizzBuzz問題を解く
fn main() {
    // 1から100まで繰り返す --- (*1)
    for i in 1..101 {
        // 条件を一つずつ判定する --- (*2)
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }
}

