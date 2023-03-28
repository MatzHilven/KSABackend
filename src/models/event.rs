use crate::{
    config::db::Connection,
    schema::events::{self, dsl::*},
};
use diesel::prelude::*;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

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

#[derive(Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = events)]
pub struct NewEvent {
    pub name: String,
    pub image_url: String,
    pub location: String,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub link: Option<String>,
}

impl Event {
    pub fn find_all(connection: &mut Connection) -> QueryResult<Vec<Event>> {
        events.order_by(id).load::<Event>(connection)
    }

    pub fn find_by_id(i: i32, connection: &mut Connection) -> QueryResult<Event> {
        events.find(i).get_result::<Event>(connection)
    }

    pub fn insert(new_event: NewEvent, connection: &mut Connection) -> QueryResult<usize> {
        diesel::insert_into(events)
            .values(&new_event)
            .execute(connection)
    }

    pub fn update(i: i32, new_event: NewEvent, connection: &mut Connection) -> QueryResult<usize> {
        diesel::update(events.find(i))
            .set(&new_event)
            .execute(connection)
    }

    pub fn delete(i: i32, connection: &mut Connection) -> QueryResult<usize> {
        diesel::delete(events.find(i)).execute(connection)
    }
}
