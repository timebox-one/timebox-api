mod routes;
mod models;
mod database;
mod auth;

#[macro_use]
extern crate diesel;
extern crate dotenv;
use actix_web::dev::ServiceRequest;
use actix_web::{middleware, Error, HttpMessage};
use actix_web::{HttpServer,App, web::Data};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::middleware::HttpAuthentication;
use routes::test::test;
use routes::index::index;
use database::mysql::establish_connection;


async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {
    let config = req
        .app_data::<Config>()
        .map(|data| data.clone())
        .unwrap_or_else(Default::default);
    if req.path().eq("/") || req.path().eq("/test") {
        return Ok(req);
    }
    println!("{}", req.path());
    match auth::validate_token(credentials.token()) {
        Ok(res) => {
            if !res.user.is_empty() {
                req.extensions_mut().insert(res);
                Ok(req)
            } else {
                Err(AuthenticationError::from(config).with_error_description("desc").into())
            }
        }
        Err(msg) => Err(AuthenticationError::from(config).with_error_description(msg).into()),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = establish_connection();

    println!("Booting up REST-Server..");
    HttpServer::new(move|| {
        let auth = HttpAuthentication::bearer(validator);
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(auth)
            .app_data(Data::new(pool.clone()))
            .service(test)
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
    /* HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await */
}
