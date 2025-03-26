use diesel::prelude::*;
use chrono::NaiveDate;
use crate::schema::victims;

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = victims)]
#[diesel(primary_key(victimid))]
pub struct Victim {
    pub victimid: i32,
    pub fullname: String,
    pub nik: Option<String>,
    pub email: Option<String>,
    pub domicileaddress: Option<String>,
    pub phonenum: Option<String>,
    pub occupation: Option<String>,
    pub sex: Option<String>,
    pub dateofbirth: Option<NaiveDate>,
    pub placeofbirth: Option<String>,
    pub officialaddress: Option<String>,
    pub educationlevel: Option<String>,
    pub faxnum: Option<String>,
    pub marriagestatus: Option<String>,
    pub marriageage: Option<i32>,
    pub isuploaded: Option<bool>,
    pub disability: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = victims)]
pub struct NewVictim {
    pub fullname: String,
    pub nik: Option<String>,
    pub email: Option<String>,
    pub domicileaddress: Option<String>,
    pub phonenum: Option<String>,
    pub occupation: Option<String>,
    pub sex: Option<String>,
    pub dateofbirth: Option<NaiveDate>,
    pub placeofbirth: Option<String>,
    pub officialaddress: Option<String>,
    pub educationlevel: Option<String>,
    pub faxnum: Option<String>,
    pub marriagestatus: Option<String>,
    pub marriageage: Option<i32>,
    pub isuploaded: Option<bool>,
    pub disability: Option<String>,
}