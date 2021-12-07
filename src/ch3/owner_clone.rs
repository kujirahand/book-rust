fn main() {
    let g1 = String::from("穏やかな心は体に良い");
    let g2 = g1.clone(); // 複製すれば所有権の移動は起きない
    println!("{}", g1); // ok
    println!("{}", g2); // ok
}

