use diesel::prelude::*;
use chrono::NaiveDateTime;
use crate::schema::updates;
use crate::model::user::User;

#[derive(Queryable, Selectable, Identifiable, Associations)]
#[diesel(belongs_to(User, foreign_key = adminid))]
#[diesel(table_name = updates)]
#[diesel(primary_key(updateid))]
pub struct Update {
    pub updateid: i32,
    pub dataid: i32,
    pub createdat: NaiveDateTime,
    pub updatedat: Option<NaiveDateTime>,
    pub remarks: Option<String>,
    pub proof: Option<String>,
    pub status: Option<String>,
    pub adminid: Option<i64>,
}

#[derive(Insertable)]
#[diesel(table_name = updates)]
pub struct NewUpdate {
    pub dataid: i32,
    pub createdat: Option<NaiveDateTime>,
    pub updatedat: Option<NaiveDateTime>,
    pub remarks: Option<String>,
    pub proof: Option<String>,
    pub status: Option<String>,
    pub adminid: Option<i64>,
}