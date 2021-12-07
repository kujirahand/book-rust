fn main() {
    // 右寄せと16進数の出力
    println!("|{:>8}| #{:06x}", "red", 0xFF0000);
    println!("|{:>8}| #{:06x}", "green", 0x00FF00);
    println!("|{:>8}| #{:06x}", "blue", 0x0000FF);
    // デバッグ出力
    println!("|{:>8}| RGB{:?}", "yellow", (255,255,0));
}

