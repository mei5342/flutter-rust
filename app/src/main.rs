use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn get_test() -> impl Responder {
    HttpResponse::Ok().body("test-ok")
}

#[get("/todos")]
async fn get_todo() -> impl Responder {
    println!("todos");
    HttpResponse::Ok().body("todos-ok")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_todo).service(get_test))
    .bind("0.0.0.0:8080")?
    .run()
    .await
}