use crate::schema::users;
use crate::schema::posts;


#[derive(Queryable)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub encrypted_password: String
}


#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub encrypted_password: &'a str 
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

