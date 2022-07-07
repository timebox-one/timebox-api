use actix_web::{HttpResponse, get, web::Data};
use diesel::{QueryDsl, RunQueryDsl};

use crate::{database::mysql::Pool, models::item::{Item, NewItem}};


#[get("/")]
pub async fn index(db: Data<Pool>) -> HttpResponse {
    /* use crate::database::schema::items::dsl::*;
    let conn = db.get().unwrap();

    let new_post = NewItem { content: &"Hello there".to_owned() };

    diesel::insert_into(items)
        .values(&new_post)
        .execute(&conn)
        .expect("Error saving new post");

    let results = items
        .limit(5)
        .load::<Item>(&conn)
        .expect("Error loading posts");
    HttpResponse::Ok().json(results) */
    HttpResponse::Ok().body("Healthy")
}
