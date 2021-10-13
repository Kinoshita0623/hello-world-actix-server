use crate::schema::users;
use crate::schema::posts;
use crate::schema::user_tokens;
use uuid::Uuid;
extern crate bcrypt;
use bcrypt::{DEFAULT_COST, hash, verify};
use serde::Serialize;
use chrono::NaiveDateTime;
use chrono::Utc;

#[derive(Queryable)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub encrypted_password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Queryable)]
pub struct SimpleUser {
    pub id: i64,
    pub username: String,
}

#[derive(Queryable)]
pub struct UserToken {
    pub id: i64,
    pub user_id: i64,
    pub token: String
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub username: String,
    pub encrypted_password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime 
}


#[derive(Queryable)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub text: String,
    pub user_id: i64,
}

#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub text: &'a str,
    pub user_id: i64
}

#[derive(Insertable)]
#[table_name="user_tokens"]
pub struct NewUserToken {
    pub user_id: i64,
    pub token: String
}


impl NewUserToken {

    pub fn new(user_id: i64) -> Self {
        return NewUserToken {
            user_id: user_id,
            token: Uuid::new_v4().to_string()
        };
    }
}

impl User {

    pub fn check_password(&self, password: String) -> bool {
        return match verify(password, &self.encrypted_password) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn set_password(&mut self, password: String) -> bool {
        return match hash(password, DEFAULT_COST) {
            Ok(encrypted_password) => {
                self.encrypted_password = encrypted_password.to_string();
                true
            }
            Err(_) => false,
        }
    }
}

impl NewUser {
    pub fn new(username: String, password: String) -> Result<Self, String>{
        return match hash(password, DEFAULT_COST) {
            Ok(encrypted_password) => {
                Ok(Self {
                    username: username,
                    encrypted_password: encrypted_password,
                    created_at: Utc::now().naive_utc(),
                    updated_at: Utc::now().naive_utc()
                })
            }
            Err(err) => Err(err.to_string()),
        }
    }
}