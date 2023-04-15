use awc::Client;

#[actix_web::main]
async fn main() {
    println!("---------app1呼び出し----------");
    let client = Client::new();
    let url = "http://rust-app-1:8080/todos";
    let res = client
        .get(url)
        .send()
        .await;
    println!("Response: {:?}", res);

    
    println!("---------flutter呼び出し----------");
    let client = Client::new();
    let url = "http://frontend:8888";
    let res = client
        .get(url)
        .send()
        .await;
    println!("Response: {:?}", res); 
}