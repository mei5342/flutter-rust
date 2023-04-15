use awc::Client;

#[actix_web::main]
async fn main() {
    let client = Client::new();
    let url = "http://rust-app-1:8080/todos";
    let res = client
        .get(url)
        .send()
        .await;
    println!("Response: {:?}", res);
}