use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::activities;

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Activity {
    pub id: i32,
    pub ban: String,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub description: String,
    pub extra: Option<String>,
}

#[derive(Deserialize)]
pub struct ActivityInput {
    pub ban: String,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub description: String,
    pub extra: Option<String>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = activities)]
pub struct NewActivity<'a> {
    pub ban: &'a str,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub description: &'a str,
    pub extra: Option<&'a str>,
}
