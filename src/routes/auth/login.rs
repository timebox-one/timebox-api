use actix_web::{HttpResponse, web, post};

use crate::{database::mysql::Pool, auth::UserToken, models::{user::{LoginUser, User}, response::ResponseBody}};

#[post("/login")]
pub async fn login(db: web::Data<Pool>, user_data: Option<web::ReqData<UserToken>>, login_data: web::Json<LoginUser>) -> HttpResponse {
    let conn = db.get().unwrap();
    match user_data {
        Some(_) => {
            return HttpResponse::BadRequest().json(ResponseBody::new(&"Already logged in".to_owned(), &"".to_owned()))
        },
        _ => {
            match User::login(login_data.0, &conn) {
                Ok(user) => {
                    let token = UserToken::generate_token(&user);
                    return HttpResponse::Ok().json(ResponseBody::new(&"Login successfull".to_owned(), &token));
                },
                Err(msg) => HttpResponse::BadRequest().json(ResponseBody::new(&msg, &"".to_owned()))
            }
        }
    }
}