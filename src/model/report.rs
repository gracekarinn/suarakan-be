use diesel::prelude::*;
use chrono::NaiveDateTime;
use crate::schema::reports;
use crate::model::reporter::Reporter;
use crate::model::incident::Incident;
use crate::model::victim::Victim;
use crate::model::accused::Accused;
use crate::model::proof::Proof;
use crate::model::update::Update;

#[derive(Queryable, Selectable, Identifiable, Associations)]
#[diesel(belongs_to(Reporter, foreign_key = reporterid))]
#[diesel(belongs_to(Incident, foreign_key = incidentid))]
#[diesel(belongs_to(Victim, foreign_key = victimid))]
#[diesel(belongs_to(Accused, foreign_key = accusedid))]
#[diesel(belongs_to(Proof, foreign_key = proofid))]
#[diesel(belongs_to(Update, foreign_key = updateid))]
#[diesel(table_name = reports)]
#[diesel(primary_key(reportid))]
pub struct Report {
    pub reportid: i32,
    pub reporterid: i32,
    pub createdat: NaiveDateTime,
    pub updatedat: Option<NaiveDateTime>,
    pub proofid: Option<i32>,
    pub incidentid: Option<i32>,
    pub victimid: Option<i32>,
    pub accusedid: Option<i32>,
    pub updateid: Option<i32>,
}

#[derive(Insertable)]
#[diesel(table_name = reports)]
pub struct NewReport {
    pub reporterid: i32,
    pub createdat: Option<NaiveDateTime>,
    pub updatedat: Option<NaiveDateTime>,
    pub proofid: Option<i32>,
    pub incidentid: Option<i32>,
    pub victimid: Option<i32>,
    pub accusedid: Option<i32>,
    pub updateid: Option<i32>,
}