use diesel::prelude::*;
use chrono::NaiveDateTime;
use chrono::NaiveDate;
use crate::schema::reports;
use crate::model::update::Update;
use crate::model::reporter::Reporter;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Selectable, Identifiable, Associations, AsChangeset, Serialize, Deserialize)]
#[diesel(belongs_to(Update, foreign_key = updateid))]
#[diesel(belongs_to(Reporter, foreign_key = reporterid))]
#[diesel(table_name = reports)]
#[diesel(primary_key(reportid))]
pub struct Report {
    pub reportid: i32,
    pub reporterid: i32,
    pub updateid: i32,
    pub createdat: Option<NaiveDateTime>,
    pub updatedat: Option<NaiveDateTime>,

    // REPORTER
    pub reporterfullname: Option<String>,
    pub reporterphonenum: Option<String>,
    // pub reporteroccupation: Option<String>,
    // pub reporterdateofbirth: Option<NaiveDate>,
    pub reporteraddress: Option<String>,
    pub reporterrelationship: Option<String>,

    // INCIDENT
    pub incidentlocation: String,
    pub incidenttime: NaiveDateTime,
    pub incidentdescription: Option<String>,
    pub incidentvictimneeds: Option<String>,
    // pub incidentpasteffort: Option<String>,
    pub incidentproof: Option<String>,

    // VICTIM
    pub victimfullname: String,
    pub victimnik: Option<String>,
    pub victimemail: Option<String>,
    pub victimaddress: Option<String>,
    pub victimphonenum: Option<String>,
    pub victimoccupation: Option<String>,
    pub victimsex: Option<String>,
    pub victimdateofbirth: Option<NaiveDate>,
    pub victimplaceofbirth: Option<String>,
    pub victimeducationlevel: Option<String>,
    pub victimmarriagestatus: Option<String>,
    // pub victimdisability: Option<String>,

    // ACCUSED
    pub accusedfullname: String,
    // pub accusedemail: Option<String>,
    pub accusedaddress: Option<String>,
    pub accusedphonenum: Option<String>,
    pub accusedoccupation: Option<String>,
    pub accusedsex: Option<String>,
    // pub accuseddateofbirth: Option<NaiveDate>,
    // pub accusedplaceofbirth: Option<String>,
    // pub accusededucationlevel: Option<String>,
    pub accusedrelationship: Option<String>,

    // AUTHORITY
    pub authority: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = reports)]
pub struct NewReport {
    pub reportid: i32,
    pub reporterid: i32,
    pub updateid: i32,
    pub createdat: Option<NaiveDateTime>,
    pub updatedat: Option<NaiveDateTime>,

    // REPORTER
    pub reporterfullname: Option<String>,
    pub reporterphonenum: Option<String>,
    // pub reporteroccupation: Option<String>,
    // pub reporterdateofbirth: Option<NaiveDate>,
    pub reporteraddress: Option<String>,
    pub reporterrelationship: Option<String>,

    // INCIDENT
    pub incidentlocation: String,
    pub incidenttime: NaiveDateTime,
    pub incidentdescription: Option<String>,
    pub incidentvictimneeds: Option<String>,
    // pub incidentpasteffort: Option<String>,
    pub incidentproof: Option<String>,

    // VICTIM
    pub victimfullname: String,
    pub victimnik: Option<String>,
    pub victimemail: Option<String>,
    pub victimaddress: Option<String>,
    pub victimphonenum: Option<String>,
    pub victimoccupation: Option<String>,
    pub victimsex: Option<String>,
    pub victimdateofbirth: Option<NaiveDate>,
    pub victimplaceofbirth: Option<String>,
    pub victimeducationlevel: Option<String>,
    pub victimmarriagestatus: Option<String>,
    // pub victimdisability: Option<String>,

    // ACCUSED
    pub accusedfullname: String,
    // pub accusedemail: Option<String>,
    pub accusedaddress: Option<String>,
    pub accusedphonenum: Option<String>,
    pub accusedoccupation: Option<String>,
    pub accusedsex: Option<String>,
    // pub accuseddateofbirth: Option<NaiveDate>,
    // pub accusedplaceofbirth: Option<String>,
    // pub accusededucationlevel: Option<String>,
    pub accusedrelationship: Option<String>,

    // AUTHORITY
    pub authority: String,
}