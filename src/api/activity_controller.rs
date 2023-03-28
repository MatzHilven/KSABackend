use actix_web::{web, HttpResponse, Result};

use crate::{
    config::db::Pool,
    constants,
    models::response::ResponseBody,
    services::activity_service
};

// GET api/activity
pub async fn find_all(pool: web::Data<Pool>) -> Result<HttpResponse> {
    match activity_service::find_all(&pool) {
        Ok(activities) => Ok(HttpResponse::Ok()
            .json(ResponseBody::new(constants::MESSAGE_OK, activities))),
        Err(err) => Ok(err.response())
    }
}
//
// // GET api/activity/{id}
// pub async fn get_activity(path_id: Path<i32>) -> HttpResponse {
//     use crate::schema::activities::dsl::*;
//
//
//     match activities
//         .filter(id.eq(path_id.into_inner()))
//         .load::<Activity>(&mut connection) {
//         Ok(res) => HttpResponse::Ok().json(res),
//         Err(_) => HttpResponse::NotFound().json("Activity not found."),
//     }
// }
//
// // PUT api/activity/{id}
// pub async fn edit_activity(path_id: Path<i32>, activity: Json<ActivityInput>) -> HttpResponse {
//     use crate::schema::activities::dsl::*;
//
//
//     let new_activity = NewActivity {
//         ban: activity.ban.as_str(),
//         start_date: activity.start_date,
//         end_date: activity.end_date,
//         description: activity.description.as_str(),
//         extra: activity.extra.as_deref(),
//     };
//
//     match diesel::update(activities.filter(id.eq(path_id.into_inner())))
//         .set(&new_activity)
//         .execute(&mut connection) {
//         Ok(_) => HttpResponse::Ok().json("Successfully updated activity."),
//         Err(err) => HttpResponse::InternalServerError().json( err.to_string()),
//     }
// }
//
// // POST api/activity/
// pub async fn add_activity(activity: Json<ActivityInput>) -> HttpResponse {
//     use crate::schema::activities::dsl::*;
//
//
//     let new_activity = NewActivity {
//         ban: activity.ban.as_str(),
//         start_date: activity.start_date,
//         end_date: activity.end_date,
//         description: activity.description.as_str(),
//         extra: activity.extra.as_deref(),
//     };
//
//     match diesel::insert_into(activities)
//         .values(&new_activity)
//         .execute(&mut connection) {
//         Ok(_) => HttpResponse::Created().json("Successfully added activity."),
//         Err(err) => HttpResponse::InternalServerError().json( err.to_string()),
//     }
// }