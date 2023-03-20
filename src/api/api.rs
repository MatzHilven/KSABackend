use actix_web::{
    get,
    post,
    HttpResponse,
    web::Json,
};

use crate::models::activity::{Activity, NewActivity};
use crate::db::db::establish_connection;
use diesel::prelude::*;

#[get("/")]
pub async fn alive() -> HttpResponse {
    HttpResponse::Ok().json("alive")
}

#[get("/activities")]
pub async fn get_activities() -> HttpResponse {
    use crate::schema::activities::dsl::*;

    let mut connection = establish_connection();

    let results = activities
        .load::<Activity>(&mut connection)
        .expect("Error loading activities");

    HttpResponse::Ok().json(results)
}

#[post("/activities")]
pub async fn add_activity(activity: Json<Activity>) -> HttpResponse {
    use crate::schema::activities::dsl::*;

    let mut connection = establish_connection();

    let new_activity = NewActivity {
        ban: activity.ban.as_str(),
        start_date: activity.start_date,
        end_date: activity.end_date,
        description: activity.description.as_str(),
        extra: activity.extra.as_deref(),
    };

    match diesel::insert_into(activities)
        .values(&new_activity)
        .execute(&mut connection) {
        Ok(_) => HttpResponse::Created().json("Successfully added activity."),
        Err(err) => HttpResponse::InternalServerError().json( err.to_string()),
    }
}