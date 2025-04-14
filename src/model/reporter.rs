use diesel::prelude::*;
use chrono::NaiveDate;
use crate::schema::reporters;
use crate::model::user::User;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(belongs_to(User, foreign_key = reporterid))]
#[diesel(table_name = reporters)]
#[diesel(primary_key(reporterid))]
pub struct Reporter {
    pub reporterid: i32,
    pub phonenum: Option<String>,
    pub occupation: Option<String>,
    pub dateofbirth: Option<NaiveDate>,
    pub officialaddress: Option<String>,
    pub faxnum: Option<String>,
    pub relationship: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = reporters)]
pub struct NewReporter {
    pub reporterid: i32,
    pub phonenum: Option<String>,
    pub occupation: Option<String>,
    pub dateofbirth: Option<NaiveDate>,
    pub officialaddress: Option<String>,
    pub faxnum: Option<String>,
    pub relationship: Option<String>,
}