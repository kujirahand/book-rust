fn main() {
    let g1 = 30;
    let g2 = g1; // 値が自動的にコピーされる --- (*1)
    println!("{}", g1); // ok
    println!("{}", g2); // ok
}

