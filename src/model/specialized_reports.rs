use diesel::prelude::*;
use chrono::NaiveDateTime;
use crate::schema::{ui_reports, perempuan_reports, ham_reports};
use crate::model::report::Report;

#[derive(Queryable, Selectable, Identifiable, Associations)]
#[diesel(belongs_to(Report, foreign_key = reportid))]
#[diesel(table_name = ui_reports)]
#[diesel(primary_key(reportid))]
pub struct UIReport {
    pub reportid: i32,
    pub updateid: Option<i32>,
}

#[derive(Insertable)]
#[diesel(table_name = ui_reports)]
pub struct NewUIReport {
    pub reportid: i32,
    pub updateid: Option<i32>,
}

#[derive(Queryable, Selectable, Identifiable, Associations)]
#[diesel(belongs_to(Report, foreign_key = reportid))]
#[diesel(table_name = perempuan_reports)]
#[diesel(primary_key(reportid))]
pub struct PerempuanReport {
    pub reportid: i32,
    pub updateid: Option<i32>,
}

#[derive(Insertable)]
#[diesel(table_name = perempuan_reports)]
pub struct NewPerempuanReport {
    pub reportid: i32,
    pub updateid: Option<i32>,
}

#[derive(Queryable, Selectable, Identifiable, Associations)]
#[diesel(belongs_to(Report, foreign_key = reportid))]
#[diesel(table_name = ham_reports)]
#[diesel(primary_key(reportid))]
pub struct HamReport {
    pub reportid: i32,
    pub updateid: Option<i32>,
}

#[derive(Insertable)]
#[diesel(table_name = ham_reports)]
pub struct NewHamReport {
    pub reportid: i32,
    pub updateid: Option<i32>,
}