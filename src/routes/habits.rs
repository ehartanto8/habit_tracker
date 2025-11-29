use actix_web::{get, post, web, HttpResponse, Responder};
use crate::models::habit::{NewHabit, Habit};
use sqlx::{Pool, Sqlite};

#[get("/habits")]
pub async fn get_habits(pool: web::Data<Pool<Sqlite>>) -> impl Responder {
    let habits = sqlx::query_as::<_, Habit>("SELECT id, description FROM habits")
        .fetch_all(pool.get_ref())
        .await;

    match habits {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(e) => {
            eprintln!("DB error: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/habits")]
pub async fn create_habit(
    pool: web::Data<Pool<Sqlite>>,
    payload: web::Json<NewHabit>,
) -> HttpResponse {
    crate::db::habit_queries::insert(&pool, &payload.description).await;

    HttpResponse::Created().finish()
}