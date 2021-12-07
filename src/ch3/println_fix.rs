fn main() {
    let s = "気前よく与えて豊かになる人がいる".to_string();
    println!("{}", s); // 所有権は移動しない
    println!("{}", s); // 
}

