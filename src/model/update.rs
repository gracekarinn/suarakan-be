use diesel::prelude::*;
use chrono::NaiveDateTime;
use crate::schema::updates;
use crate::model::user::User;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Selectable, Identifiable, Associations, AsChangeset, Serialize, Deserialize)]
#[diesel(belongs_to(User, foreign_key = adminid))]
#[diesel(table_name = updates)]
#[diesel(primary_key(updateid))]
pub struct Update {
    pub updateid: i32,
    pub createdat: NaiveDateTime,
    pub updatedat: Option<NaiveDateTime>,
    pub remarks: Option<String>,
    pub proof: Option<String>,
    pub status: Option<String>,
    pub adminid: Option<i64>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = updates)]
pub struct NewUpdate {
    pub updateid: i32,
    pub createdat: NaiveDateTime,
    pub updatedat: Option<NaiveDateTime>,
    pub remarks: Option<String>,
    pub proof: Option<String>,
    pub status: Option<String>,
    pub adminid: Option<i64>,
}