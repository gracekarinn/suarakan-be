use diesel::prelude::*;
use chrono::NaiveDateTime;
use crate::schema::publications;
use crate::model::admin::Admin;

#[derive(Queryable, Selectable, Identifiable, Associations, AsChangeset)]
#[diesel(belongs_to(Admin, foreign_key = adminid))]
#[diesel(table_name = publications)]
#[diesel(primary_key(publicationid))]
pub struct Publication {
    pub publicationid: i32,
    pub title: String,
    pub createdat: NaiveDateTime,
    pub updatedat: Option<NaiveDateTime>,
    pub description: Option<String>,
    pub filelink: Option<String>,
    pub adminid: Option<i32>,
}

#[derive(Insertable)]
#[diesel(table_name = publications)]
pub struct NewPublication {
    pub title: String,
    pub createdat: NaiveDateTime,
    pub updatedat: Option<NaiveDateTime>,
    pub description: Option<String>,
    pub filelink: Option<String>,
    pub adminid: Option<i32>,
}