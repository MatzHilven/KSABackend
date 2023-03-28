#[allow(unused_imports)]
use diesel::{
    pg::PgConnection,
    r2d2::{self, ConnectionManager},
    sql_query,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub type Connection = PgConnection;
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[cfg(not(test))]
pub fn migrate_and_config_db(url: &str) -> Pool {
    let manager = ConnectionManager::<Connection>::new(url);
    let pool = Pool::builder().test_on_check_out(true).build(manager).expect("Failed to create pool.");

    let mut connection = pool.get().unwrap();
    connection.run_pending_migrations(MIGRATIONS).expect("Failed to run migrations.");

    pool
}

#[cfg(test)]
pub fn migrate_and_config_db(url: &str) -> Pool {
    use crate::diesel::RunQueryDsl;
    let manager = ConnectionManager::<Connection>::new(url);
    Pool::builder().test_on_check_out(true).build(manager).expect("Failed to create pool.");

    sql_query(r#"DROP TABLE IF EXISTS login_history;"#).execute(&mut pool.get().unwrap()).expect("TODO: panic message");
    sql_query(r#"DROP TABLE IF EXISTS users;"#).execute(&mut pool.get().unwrap()).expect("TODO: panic message");
    sql_query(r#"DROP TABLE IF EXISTS activities;"#).execute(&mut pool.get().unwrap()).expect("TODO: panic message");
    sql_query(r#"DROP TABLE IF EXISTS events;"#).execute(&mut pool.get().unwrap()).expect("TODO: panic message");
    sql_query(
        r#"CREATE TABLE activities
(
    id          SERIAL    NOT NULL,
    ban         TEXT      NOT NULL,
    start_date  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    end_date    TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    description TEXT      NOT NULL,
    extra       TEXT,
    CONSTRAINT activities_pkey PRIMARY KEY (id)
);
"#,
    )
        .execute(&mut pool.get().unwrap()).expect("TODO: panic message");
    sql_query(
        r#"CREATE TABLE events
(
    id         SERIAL    NOT NULL,
    name       TEXT      NOT NULL,
    image_url  TEXT      NOT NULL,
    location   TEXT      NOT NULL,
    start_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    end_date   TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    link       TEXT,
    CONSTRAINT events_pkey PRIMARY KEY (id)
);
"#,
    )
        .execute(&mut pool.get().unwrap()).expect("TODO: panic message");
    sql_query(
        r#"CREATE TABLE users (
        id INTEGER PRIMARY KEY NOT NULL,
        username TEXT NOT NULL,
        email TEXT NOT NULL,
        password TEXT NOT NULL,
        login_session TEXT NOT NULL DEFAULT ''
    );"#,
    )
        .execute(&mut pool.get().unwrap()).expect("TODO: panic message");
    sql_query(
        r#"CREATE TABLE login_history (
        id INTEGER PRIMARY KEY NOT NULL,
        user_id INTEGER NOT NULL REFERENCES users(id),
        login_timestamp INTEGER NOT NULL
    );"#,
    )
        .execute(&mut pool.get().unwrap()).expect("TODO: panic message");

    pool
}