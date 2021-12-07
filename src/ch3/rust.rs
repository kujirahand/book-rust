fn main() {
    let pr = "Rust";
    // 1バイトずつ表示 --- (*1)
    for c in pr.bytes() {
        print!("{:2x} ", c);
    }
}

