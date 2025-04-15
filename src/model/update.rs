use diesel::prelude::*;
use chrono::NaiveDateTime;
use crate::schema::updates;
use crate::model::report::Report;
use serde::{Serialize, Deserialize};

#[derive(Debug, Queryable, Selectable, Identifiable, Associations, AsChangeset, Serialize, Deserialize)]
#[diesel(belongs_to(Report, foreign_key = reportid))]
#[diesel(table_name = updates)]
#[diesel(primary_key(updateid))]
pub struct Update {
    pub updateid: i32,
    pub createdat: NaiveDateTime,
    pub updatedat: Option<NaiveDateTime>,
    pub remarks: Option<String>,
    pub proof: Option<String>,
    pub status: Option<String>,
    pub reportid: i32,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = updates)]
pub struct NewUpdate {
    pub createdat: NaiveDateTime,
    pub updatedat: Option<NaiveDateTime>,
    pub remarks: Option<String>,
    pub proof: Option<String>,
    pub status: Option<String>,
    pub reportid: i32,
}