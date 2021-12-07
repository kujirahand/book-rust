fn main() {
    let pr = "豚に真珠";
    // 1バイトずつ表示 --- (*1)
    for c in pr.bytes() {
        print!("{:2x} ", c);
    }
}

