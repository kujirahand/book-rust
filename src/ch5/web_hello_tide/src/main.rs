// サーバーアドレスとポートを指定 --- (*1)
const SERVER_ADDR: &str = "127.0.0.1:8888";

// メイン関数 --- (*2)
#[async_std::main]
async fn main() -> tide::Result<()> {
    println!("http://{}/", SERVER_ADDR);
    // Tideのオブジェクトを作成 --- (*3)
    let mut app = tide::new();
    // ルーティングを指定 --- (*4)
    app.at("/").get(|_| async { Ok("Hello, World!") });
    // サーバーを起動 --- (*5)
    app.listen(SERVER_ADDR).await?;
    Ok(())
}

