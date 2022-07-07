use diesel::{prelude::*, r2d2::ConnectionManager};
use dotenv::dotenv;
use std::env;

pub type Pool = diesel::r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub fn establish_connection() -> Pool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    diesel::r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.")
}