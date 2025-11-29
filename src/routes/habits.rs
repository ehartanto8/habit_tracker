use actix_web::{get, post, web, HttpResponse};
use crate::models::habit::NewHabit;
use sqlx::{Pool, Sqlite};

pub async fn placeholder() -> HttpResponse {
    HttpResponse::Ok().body("API is working")
}

#[post("/habits")]
pub async fn create_habit(
    pool: web::Data<Pool<Sqlite>>,
    payload: web::Json<NewHabit>,
) -> HttpResponse {
    crate::db::habit_queries::insert(&pool, &payload.description).await;

    HttpResponse::Created().finish()
}