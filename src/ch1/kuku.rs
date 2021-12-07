// Rustで九九の表を作成
fn main() {
    for y in 1..10 {
        for x in 1..10 {
            print!("{:3},", x * y);
        }
        println!("");
    }
}

