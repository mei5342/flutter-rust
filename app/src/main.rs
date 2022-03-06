// wsl2 localhostアクセスエラー 参考：https://qiita.com/snaka/items/a8eee4cfc8f7d733e6ab

// docker実行
// docker build -t my-rust-app .
// docker run -it --rm -p 8080:8080 --name my-running-app my-rust-app

// アクセス
// http://localhost:8080/app/index.html

use actix_web::{web, App, HttpServer, Responder};

async fn index() -> impl Responder {
    "Hello world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            // prefixes all resources and routes attached to it...
            web::scope("/app")
                // ...so this handles requests for `GET /app/index.html`
                .route("/index.html", web::get().to(index)),
        )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}