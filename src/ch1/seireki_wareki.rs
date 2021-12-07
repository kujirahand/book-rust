fn main() {
    for y in 1926..2027 {
        // 西暦を改行なしで出力
        print!("西暦{}年 = ", y);
        // 和暦を判定して改行付きで出力
        if y >= 2019 {
            if y == 2019 { println!("令和元年"); }
            else { println!("令和{}年", y-2019+1); }
        } else if y >= 1989 {
            if y == 1989 { println!("平成元年"); }
            else { println!("平成{}年", y-1989+1); }
        } else if y >= 1926 {
            if y == 1926 { println!("昭和元年"); }
            else { println!("昭和{}年", y-1926+1); }
        }
    }
}

