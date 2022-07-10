use actix_web::{HttpResponse, web, post};
use crate::{database::mysql::Pool, auth::UserToken, models::{user::{NewUser, User}, response::ResponseBody}};

#[post("/signup")]
pub async fn signup(db: web::Data<Pool>, user_data: Option<web::ReqData<UserToken>>, req_data: web::Json<NewUser>) -> HttpResponse {
    let conn = db.get().unwrap();

    match user_data {
        Some(_) => {
            return HttpResponse::BadRequest().json(ResponseBody::new(&"Already logged in".to_owned(), &"".to_owned()))
        },
        _ => {
            match User::create_user(req_data.0, &conn) {
                Ok(msg) => HttpResponse::Created().json(ResponseBody::new(&msg, &"".to_owned())),
                Err(msg) => HttpResponse::BadRequest().json(ResponseBody::new(&msg, &"".to_owned()))
            }
        }
    }
}