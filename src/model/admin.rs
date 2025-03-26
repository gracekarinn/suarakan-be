use diesel::prelude::*;
use crate::schema::admins;
use crate::model::user::User;

#[derive(Queryable, Selectable, Identifiable, Associations)]
#[diesel(belongs_to(User, foreign_key = adminid))]
#[diesel(table_name = admins)]
#[diesel(primary_key(adminid))]
pub struct Admin {
    pub adminid: i32,
}

#[derive(Insertable)]
#[diesel(table_name = admins)]
pub struct NewAdmin {
    pub adminid: i32,
}