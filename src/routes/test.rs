use actix_web::web;
use actix_web::{web::Data, HttpResponse, get};
use diesel::QueryDsl;
use diesel::prelude::*;

use crate::auth::UserToken;
use crate::models::item::NewItem;
use crate::models::user::{User, NewUser};
use crate::{database::{mysql::Pool}, models::item::Item};

#[get("/test")]
pub async fn test(db: Data<Pool>, userData: Option<web::ReqData<UserToken>>) -> HttpResponse {
    use crate::database::schema::users::dsl::*;
    let conn = db.get().unwrap();

    let new_User = NewUser{ email: "asd", username: "secure", password: "username"};

    diesel::insert_into(users)
        .values(&new_User)
        .execute(&conn)
        .expect("Error saving new user");

    let results = users
        .limit(1)
        .get_result::<User>(&conn);
    match results {
        Ok(result_items) => {
            match userData {
                Some(_) => {
                    return HttpResponse::Ok().body("Already logged in")
                },
                _ => {
                    let token = UserToken::generate_token(&result_items);
                    return HttpResponse::Ok().json(token);
                }
            }
        },
        Err(_) => return HttpResponse::InternalServerError().body("Error retrieving")
    }
    /* match userData {
        Some(utoken) => {
            if let Ok(user) = User::find_user_by_username(&utoken.user, &conn) {
                let token = UserToken::generate_token(&user);
                return HttpResponse::Ok().json(token);
            }
            return HttpResponse::InternalServerError().body("Error1");
        },
        _ => return HttpResponse::InternalServerError().body("Erorr2")
    }  */
    
}