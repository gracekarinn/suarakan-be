use diesel::prelude::*;
use crate::schema::users;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = users)]
#[diesel(primary_key(userid))]
pub struct User {
    pub userid: i32,
    pub fullname: String,
    pub email: String,
    pub password: String,
    pub role: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub fullname: String,
    pub email: String,
    pub password: String,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JwtClaims {
    pub token_type: String,
    pub exp: usize,
    pub iat: usize,
    pub jti: String,
    pub user_id: i32,
    pub email: String,
    pub full_name: String,
    pub user_type: String, 
    pub is_email_verified: bool,
}