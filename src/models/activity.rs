use crate::{
    config::db::Connection,
    schema::activities::{self, dsl::*},
};
use diesel::prelude::*;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Activity {
    pub id: i32,
    pub ban: String,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub description: String,
    pub extra: Option<String>,
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = activities)]
pub struct ActivityDTO {
    pub ban: String,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub description: String,
    pub extra: Option<String>,
}

impl Activity {
    pub fn find_all(connection: &mut Connection) -> QueryResult<Vec<Activity>> {
        activities.order_by(id).load::<Activity>(connection)
    }

    pub fn find_by_id(i: i32, connection: &mut Connection) -> QueryResult<Activity> {
        activities.find(i).get_result::<Activity>(connection)
    }

    pub fn insert(new_activity: ActivityDTO, connection: &mut Connection) -> QueryResult<usize> {
        diesel::insert_into(activities)
            .values(&new_activity)
            .execute(connection)
    }

    pub fn update(i: i32, new_activity: ActivityDTO, connection: &mut Connection) -> QueryResult<usize> {
        diesel::update(activities.find(i))
            .set(&new_activity)
            .execute(connection)
    }

    pub fn delete(i: i32, connection: &mut Connection) -> QueryResult<usize> {
        diesel::delete(activities.find(i)).execute(connection)
    }
}
