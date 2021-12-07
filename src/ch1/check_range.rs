fn main() {
    println!("--- 符号あり整数 ---");
    println!("i8={}~{}", i8::MIN, i8::MAX);
    println!("i16={}~{}", i16::MIN, i16::MAX);
    println!("i32={}~{}", i32::MIN, i32::MAX);
    println!("i64={}~{}", i64::MIN, i64::MAX);
    
    println!("--- 符号なし整数 ---");
    println!("u8={}~{}", u8::MIN, u8::MAX);
    println!("u16={}~{}", u16::MIN, u16::MAX);
    println!("u32={}~{}", u32::MIN, u32::MAX);
    println!("u64={}~{}", u64::MIN, u64::MAX);
    
    println!("--- 利用環境に応じて変わる整数 --- ");
    println!("isize={}~{}", isize::MIN, isize::MAX);
    println!("usize={}~{}", usize::MIN, usize::MAX);
    println!("usize=u{}", usize::BITS);
}

