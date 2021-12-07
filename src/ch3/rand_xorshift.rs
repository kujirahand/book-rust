use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // 乱数を初期化 --- (*1)
    let mut seed = rand_init();
    // 100個の乱数を表示 --- (*2)
    for _ in 0..100 {
        // 乱数を生成 --- (*3)
        let v = rand(&mut seed, 1, 6);
        println!("{}", v);
    }
}

// 乱数を初期化する関数 --- (*4)
fn rand_init() -> u32 {
    // 現在時刻を利用して乱数を初期化
    SystemTime::now()
      .duration_since(UNIX_EPOCH).unwrap()
      .as_millis() as u32
}

// startからendの乱数を生成する関数 --- (*5)
fn rand(seed: &mut u32, start: u32, end: u32) -> u32 {
    *seed ^= *seed << 13;
    *seed ^= *seed >> 17;
    *seed ^= *seed << 5;
    return *seed % (end - start + 1) + start;
}


