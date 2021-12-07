fn main() {
    hex_dump("自分の口を見張る人は自分の命を守る。");
}

fn hex_dump(s: &str) {
    // １バイトずつ表示する --- (*1)
    for (i, c) in s.bytes().enumerate() {
        // アドレスを表示
        if i % 16 == 0 {
            print!("{:08x}|", i);
        }
        // 4桁ごとに区切り文字を入れる
        if i % 4 == 3 {
            print!("{:02x}|", c);
        } else {
            print!("{:02x} ", c);
        }
        // 16バイトごとに改行を入れる
        if i % 16 == 15 {
            println!("");
        }
    }
    println!("");
}

