#[get("/html")]
async fn html() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body("<html><head><title>Turreta.com</title></head><body><h1>Hello from Turreta.com</h1></body></html>")
}
 
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        .service(html)
    )
        // .bind("0.0.0.0:8080")?
        .bind(("0.0.0.0",8080))?
        .run()
        .await
}