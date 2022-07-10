use actix_web::web;
use actix_web::{web::Data, HttpResponse, get};

use crate::auth::UserToken;
use crate::models::user::{User};
use crate::database::{mysql::Pool};

#[get("/test")]
pub async fn test(db: Data<Pool>, user_data: Option<web::ReqData<UserToken>>) -> HttpResponse {
    let conn = db.get().unwrap();
    match user_data {
        Some(utoken) => {
            if let Ok(user) = User::find_user_by_username(&utoken.user, &conn) {
                let token = UserToken::generate_token(&user);
                return HttpResponse::Ok().json(token);
            }
            return HttpResponse::InternalServerError().body("Error1");
        },
        _ => return HttpResponse::InternalServerError().body("Erorr2")
    }
    
}