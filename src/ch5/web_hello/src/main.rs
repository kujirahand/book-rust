use actix_web::{web, App, HttpServer, HttpRequest};

// アドレスとポートの指定 --- (*1)
const SERVER_ADDR: &str = "127.0.0.1:8888";

// Actix Webのメイン関数 --- (*2)
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("[SERVER] http://{}/", SERVER_ADDR);
    // HTTPサーバーを起動 --- (*3)
    HttpServer::new(|| {
        // ルーティングを指定 --- (*4)
        App::new()
            .route("/", web::get().to(index))
    })
    .bind(SERVER_ADDR)? 
    .run()
    .await
}

// 実行される関数 --- (*5)
async fn index(req: HttpRequest) -> &'static str {
    println!("request: {:?}", req);
    "Hello, World!"
}

