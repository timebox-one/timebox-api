
use bcrypt::{DEFAULT_COST, hash, verify};
use diesel::{r2d2::{PooledConnection, ConnectionManager}, MysqlConnection, QueryResult, prelude::*};
use serde::{Serialize, Deserialize};
use crate::{database::schema::users::{self, dsl::*}};


#[derive(Serialize, Deserialize, Debug, Queryable, Identifiable)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub password: String,
    pub email: String,
}

impl User {
    pub fn update_user(&self, new_data: NewUser, conn: &PooledConnection<ConnectionManager<MysqlConnection>>) -> Result<String, String> {
        if Self::find_user_by_username(&self.username, conn).is_ok() {
            if let Ok(_) = diesel::update(self).set((password.eq(new_data.password), email.eq(new_data.email))).execute(conn) {
                return Ok("Password/Email successfully saved".to_owned());
            }
        }
        Err("User not found".to_owned())
    }


    pub fn find_user_by_username(name: &str, conn: &PooledConnection<ConnectionManager<MysqlConnection>>) -> QueryResult<User> {
        users.filter(username.eq(name)).get_result::<User>(conn)
    }

    pub fn find_user_by_email(mail: &str, conn: &PooledConnection<ConnectionManager<MysqlConnection>>) -> QueryResult<User> {
        users.filter(email.eq(mail)).get_result::<User>(conn)
    }

    pub fn create_user(user: NewUser, conn: &PooledConnection<ConnectionManager<MysqlConnection>>) -> Result<String, String> {
        if Self::find_user_by_username(&user.username, conn).is_err() {
            let hashed_pwd = hash(&user.password, DEFAULT_COST).unwrap();
            let user = NewUser {
                password: hashed_pwd,
                ..user
            };
            let _ = diesel::insert_into(users).values(&user).execute(conn);
            Ok(format!("User '{}' successfully registered", &user.username))
        } else {
            Err(format!("User '{}' is already registered", &user.username))
        }
    }

    pub fn login(user: LoginUser, conn: &PooledConnection<ConnectionManager<MysqlConnection>>) -> Result<User, String> {
        if let Ok(user_candidate) = users
            .filter(username.eq(&user.username_or_email))
            .or_filter(email.eq(&user.username_or_email))
            .get_result::<User>(conn) 
        {
            if !user_candidate.password.is_empty() && verify(&user.password, &user_candidate.password,).unwrap() {
                return Ok(user_candidate);
            }
        }
        Err("Password and/or username/email is incorret".to_owned())
    }
}

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginUser {
    pub username_or_email: String,
    pub password: String,
}