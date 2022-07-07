mod routes;
mod models;
mod database;

#[macro_use]
extern crate diesel;
extern crate dotenv;
use actix_web::{HttpServer,App, web::Data};
use routes::index::index;
use routes::test::test;
use database::mysql::establish_connection;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = establish_connection();

    println!("Booting up REST-Server..");
    HttpServer::new(move|| {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(index)
            .service(test)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}