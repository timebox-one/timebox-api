use actix_web::{web::Data, HttpResponse, get};

use crate::database::mysql::Pool;

#[get("/test")]
pub async fn test(db: Data<Pool>) -> HttpResponse {
    use crate::database::schema::items::dsl::*;
    let conn = db.get().unwrap();

    //let new_post = NewItem { content: &"Hello there".to_owned() };

    /* diesel::insert_into(items)
        .values(&new_post)
        .execute(&conn)
        .expect("Error saving new post"); */

    let results = items
        .limit(5)
        .load::<Item>(&conn);
    match results {
        Ok(result_items) => HttpResponse::Ok().json(result_items),
        Err(_) => HttpResponse::InternalServerError().body("Err")
    }
}