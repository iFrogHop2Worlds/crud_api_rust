pub mod mylib;
use mylib::*;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let address = dotenv::var("address").unwrap();
    let db_url = dotenv::var("db_url").unwrap();

    let listener = TcpListener::bind("127.0.0.1:8080").expect("fail to bind to the listner");

    let mut client_options = ClientOptions::parse(db_url).await.expect("failed to connect to thous server");
    client_options.app_name = Some("App".to_string());
    let client = Client::with_options(client_options).expect(" ");
    let db = client.database("Tsuga-Genie");
    let _c = db.collection::<User>("Rust_Users");
    create_name_unique(&client).await;
    println!("server is running at {}",address);
    run(listener, db).await?.await
}