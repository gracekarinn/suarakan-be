use diesel::prelude::*;
use crate::schema::authentication_user;

#[derive(Queryable, Selectable, Identifiable, Debug)]
#[diesel(table_name = authentication_user)]
#[diesel(primary_key(id))]
pub struct User {
    pub id: i64,
    pub password: String,
    pub last_login: Option<chrono::DateTime<chrono::Utc>>,
    pub is_superuser: bool,
    pub first_name: String,
    pub last_name: String,
    pub is_staff: bool,
    pub is_active: bool,
    pub date_joined: chrono::DateTime<chrono::Utc>,
    pub email: String,
    pub user_type: String,
    pub is_email_verified: bool,
    pub full_name: String,
    pub phone_number: String,
}