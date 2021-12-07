fn main() {
    let points:[i32;5] = [80, 40, 50, 90, 84];
    print_array(&points);
}
fn print_array(e: &[i32;5]) {
    println!("{:?}", e); // 要素一覧を表示
    println!("len={}", e.len()); // 要素数を表示
}

