use serde::{Serialize, Deserialize};
use crate::database::schema::items;

use super::user::User;

#[derive(Serialize, Deserialize, Debug, Queryable)]
pub struct Item {
    pub id: i32,
    pub content: String,
    pub is_important: bool,
    pub user: User,
}

#[derive(Insertable, Debug)]
#[table_name = "items"]
pub struct NewItem<'a> {
    pub content: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Day {
    pub id: i64,
    pub items: Vec<Item>
}