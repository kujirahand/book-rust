// メイン関数 --- (*1)
#[tokio::main]
async fn main() {
    // 非同期関数を準備 --- (*2)
    let f = say_later("諦めるのに時がある。");
    
    // メッセージを表示 --- (*3)
    println!("捜すのに時がある。");
    
    // 非同期処理を実行 --- (*4)
    f.await;
}

// 非同期関数を記述 --- (*5)
async fn say_later(msg: &'static str) {
    println!("{}", msg);
}

