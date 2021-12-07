use tide::prelude::*;
const SERVER_ADDR: &str = "127.0.0.1:8888";

// ユーザー情報を表す構造体を定義 --- (*1)
#[derive(Deserialize, Serialize)]
struct UserInfo {
    name: String,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    println!("http://{}/", SERVER_ADDR);
    let mut app = tide::new();
    // ルーティングを指定 --- (*2)
    app.at("/").get(|_| async { // ルートにアクセスした時
        // HTMLを出力
        Ok(tide::Response::builder(200)
            .content_type(tide::http::mime::HTML)
            .body(format!("{}{}{}{}",
                "<html><body><form action='hello'>",
                "name: <input name='name' value='kujira'>",
                "<input type='submit' value='送信'>",
                "</form></body></html>"))
            .build())
    });
    // "/hello"にアクセスした時の処理 --- (*3)
    app.at("/hello").get(|req: tide::Request<()>| async move {
        // クエリを解析して構造体に当てはめる --- (*4)
        let user: UserInfo = req.query()?;
        Ok(tide::Response::builder(200)
           .content_type(tide::http::mime::HTML)
           .body(format!("<h1>Hello, {}</h1>", user.name))
           .build())
    });
    // サーバーを起動
    app.listen(SERVER_ADDR).await?;
    Ok(())
}

