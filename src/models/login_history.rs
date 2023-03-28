use crate::{
    config::db::Connection,
    models::user::User,
    schema::login_history::{self, dsl::*},
};
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;

#[derive(Identifiable, Associations, Queryable)]
#[diesel(belongs_to(User))]
#[diesel(table_name = login_history)]
pub struct LoginHistory {
    pub id: i32,
    pub user_id: i32,
    pub login_timestamp: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = login_history)]
pub struct LoginHistoryInsertableDTO {
    pub user_id: i32,
    pub login_timestamp: NaiveDateTime,
}

impl LoginHistory {
    pub fn create(un: &str, connection: &mut Connection) -> Option<LoginHistoryInsertableDTO> {
        if let Ok(user) = User::find_user_by_username(un, connection) {
            let now = Utc::now();
            Some(LoginHistoryInsertableDTO {
                user_id: user.id,
                login_timestamp: now.naive_utc(),
            })
        } else {
            None
        }
    }

    pub fn save_login_history(insert_record: LoginHistoryInsertableDTO, connection: &mut Connection, ) -> QueryResult<usize> {
        diesel::insert_into(login_history)
            .values(&insert_record)
            .execute(connection)
    }
}
