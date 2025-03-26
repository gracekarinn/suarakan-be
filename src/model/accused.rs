use diesel::prelude::*;
use chrono::NaiveDate;
use crate::schema::accused;

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = accused)]
#[diesel(primary_key(accusedid))]
pub struct Accused {
    pub accusedid: i32,
    pub fullname: String,
    pub email: Option<String>,
    pub domicileaddress: Option<String>,
    pub phonenum: Option<String>,
    pub occupation: Option<String>,
    pub sex: Option<String>,
    pub dateofbirth: Option<NaiveDate>,
    pub placeofbirth: Option<String>,
    pub educationlevel: Option<String>,
    pub relationship: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = accused)]
pub struct NewAccused {
    pub fullname: String,
    pub email: Option<String>,
    pub domicileaddress: Option<String>,
    pub phonenum: Option<String>,
    pub occupation: Option<String>,
    pub sex: Option<String>,
    pub dateofbirth: Option<NaiveDate>,
    pub placeofbirth: Option<String>,
    pub educationlevel: Option<String>,
    pub relationship: Option<String>,
}