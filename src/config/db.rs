#[allow(unused_imports)]
use diesel::{
    pg::PgConnection,
    r2d2::{self, ConnectionManager},
    sql_query,
};

pub type Connection = PgConnection;
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[cfg(not(test))]
pub fn migrate_and_config_db(url: &str) -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(url);
    let pool = Pool::builder().build(manager)
        .expect("Failed to create pool.");
    // embedded_migrations::run(&pool.get().expect("Failed to migrate."));

    pool
}

// #[cfg(test)]
// pub fn migrate_and_config_db(url: &str) -> Pool {
//     use crate::diesel::RunQueryDsl;
//     let manager = ConnectionManager::<PgConnection>::new(url);
//     let pool = Pool::builder()
//         .build(manager)
//         .expect("Failed to create pool.");
//
//     sql_query(r#"DROP TABLE IF EXISTS login_history;"#).execute(&mut pool.get().unwrap()).expect("TODO: panic message");
//     sql_query(r#"DROP TABLE IF EXISTS users;"#).execute(&mut pool.get().unwrap()).expect("TODO: panic message");
//     sql_query(r#"DROP TABLE IF EXISTS people;"#).execute(&mut pool.get().unwrap()).expect("TODO: panic message");
//     sql_query(
//         r#"CREATE TABLE people (
//         id INTEGER PRIMARY KEY NOT NULL,
//         name TEXT NOT NULL,
//         gender BOOLEAN NOT NULL,
//         age INTEGER NOT NULL,
//         address TEXT NOT NULL,
//         phone TEXT NOT NULL,
//         email TEXT NOT NULL
//     );"#,
//     )
//         .execute(&mut pool.get().unwrap()).expect("TODO: panic message");
//     sql_query(
//         r#"CREATE TABLE users (
//         id INTEGER PRIMARY KEY NOT NULL,
//         username TEXT NOT NULL,
//         email TEXT NOT NULL,
//         password TEXT NOT NULL,
//         login_session TEXT NOT NULL DEFAULT ''
//     );"#,
//     )
//         .execute(&mut pool.get().unwrap()).expect("TODO: panic message");
//     sql_query(
//         r#"CREATE TABLE login_history (
//         id INTEGER PRIMARY KEY NOT NULL,
//         user_id INTEGER NOT NULL REFERENCES users(id),
//         login_timestamp INTEGER NOT NULL
//     );"#,
//     )
//         .execute(&mut pool.get().unwrap()).expect("TODO: panic message");
//
//     pool
// }