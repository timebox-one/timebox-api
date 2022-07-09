use serde::{Serialize, Deserialize};
use crate::database::schema::items;

#[derive(Serialize, Deserialize, Debug, Queryable)]
#[diesel(belongs_to(User))]
pub struct Item {
    pub id: u64,
    pub content: String,
    pub user_id: u64,
    pub is_important: bool,
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