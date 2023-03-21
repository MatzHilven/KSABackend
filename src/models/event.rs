use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::events;

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Event {
    pub id: i32,
    pub name: String,
    pub image_url: String,
    pub location: String,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub link: Option<String>,
}

#[derive(Deserialize)]
pub struct EventInput {
    pub name: String,
    pub image_url: String,
    pub location: String,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub link: Option<String>,
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = events)]
pub struct NewEvent<'a> {
    pub name: &'a str,
    pub image_url: &'a str,
    pub location: &'a str,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub link: Option<&'a str>,
}
