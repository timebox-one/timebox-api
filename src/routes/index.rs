use actix_web::{HttpResponse, web::Data, get};
use crate::database::mysql::Pool;


#[get("/")]
pub async fn index(_db: Data<Pool>) -> HttpResponse {
    HttpResponse::Ok().body("Ok")
}
