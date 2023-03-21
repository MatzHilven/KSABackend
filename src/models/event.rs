use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::events;

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Event {
    pub id: i32,
    pub name: String,
    pub image_url: String,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub link: Option<String>,
}

pub struct EventInput {
    pub name: String,
    pub image_url: String,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub link: Option<String>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = events)]
pub struct NewEvent<'a> {
    pub name: &'a str,
    pub image_url: &'a str,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub link: Option<&'a str>,
}
