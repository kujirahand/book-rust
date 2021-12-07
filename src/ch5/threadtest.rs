use std::{thread, time};

// 1秒毎にメッセージを3回表示する関数 --- (*1)
fn sleep_print(name: &str) {
    for i in 1..=3 {
        println!("{}: i={}", name, i);
        thread::sleep(time::Duration::from_millis(1000));
    }
}

fn main() {
    // スレッドなしの場合 --- (*2)
    println!("--- スレッドなし ---");
    sleep_print("スレッドなし");
    
    // スレッドを使う場合 --- (*3)
    println!("--- スレッドを利用 ---");
    // スレッド1
    thread::spawn(|| {
        sleep_print("次郎")
    });
    // スレッド2
    thread::spawn(|| {
        sleep_print("三郎")
    });
    // メインスレッド
    sleep_print("太郎");
}


