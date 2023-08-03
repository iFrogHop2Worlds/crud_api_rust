use actix_web:: {web, App, HttpServer};
use mongodb::Database;
//pub use actix_web::rt::net::TcpListener;
pub use std::net::TcpListener;
use actix_web::dev:: Server;
#[path = "./server_routes/user_api.rs"]
pub mod user_api;
pub use user_api::deleteUser;
pub use user_api::getUser;
pub use user_api::postUser;
pub use user_api::updateUser;



pub async fn run(listener:TcpListener, db:Database)->Result<Server, std::io:: Error> {
    let db = web::Data::new(db);
    let _server = HttpServer::new(move|| {
        App::new()
        .route("/postUser", web::post().to(postUser))
        .route("/updateUser", web::get().to(updateUser))                                               
        .route("/deleteUser", web::delete().to(deleteUser))
        .route("/getUser", web::get().to(getUser))
        .app_data(db.clone())
        })  //you could use patch method also
        .listen(listener).expect(" ")    
        .run();
    Ok(_server)
} 