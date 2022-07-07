use serde::{Serialize, Deserialize};
use crate::database::schema::users;


#[derive(Serialize, Deserialize, Debug, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub email: &'a str,
}