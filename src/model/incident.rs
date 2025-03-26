use diesel::prelude::*;
use chrono::NaiveDateTime;
use crate::schema::incidents;

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = incidents)]
#[diesel(primary_key(incidentid))]
pub struct Incident {
    pub incidentid: i32,
    pub location: String,
    pub time: NaiveDateTime,
    pub description: Option<String>,
    pub victimneeds: Option<String>,
    pub pasteffort: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = incidents)]
pub struct NewIncident {
    pub location: String,
    pub time: NaiveDateTime,
    pub description: Option<String>,
    pub victimneeds: Option<String>,
    pub pasteffort: Option<String>,
}