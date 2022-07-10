use actix_web::{HttpResponse, web, post};

use crate::{database::mysql::Pool, models::{user::{NewUser, User}, response::ResponseBody}, auth::UserToken};

#[post("/user")]
pub async fn update_user(db: web::Data<Pool>, user_data: Option<web::ReqData<UserToken>>, req_data: web::Json<NewUser>) -> HttpResponse {
    let conn = db.get().unwrap();
    match user_data {
        Some(uname) => {
            if let Ok(cuser) = User::find_user_by_username(&uname.user, &conn) {
                match cuser.update_user(req_data.0, &conn) {
                    Ok(msg) => return HttpResponse::Ok().json(ResponseBody::new(&msg, &"".to_owned())),
                    Err(msg) => return HttpResponse::BadRequest().json(ResponseBody::new(&msg, &"".to_owned()))
                }
            }
            return HttpResponse::InternalServerError().json(ResponseBody::new(&"Error retrieving user".to_owned(), &"".to_owned()))
        },
        _ => return HttpResponse::Unauthorized().json(ResponseBody::new(&"Not logged in".to_owned(), &"".to_owned()))
    }
}