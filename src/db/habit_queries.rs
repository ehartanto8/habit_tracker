use sqlx::{Pool, Sqlite};
use crate::models::habit::Habit;

pub async fn get_all(pool: &Pool<Sqlite>) -> Vec<Habit> {
    sqlx::query_as::<_, Habit>(
        "SELECT id, description, done FROM habits"
    )
        .fetch_all(pool)
        .await
        .expect("Failed to fetch habits")
}

pub async fn insert(pool: &Pool<Sqlite>, description: &str) {
    sqlx::query(
        "INSERT INTO habits (description, done) VALUES (?, 0)"
    )
        .bind(description)
        .execute(pool)
        .await
        .expect("Failed to insert habit");
}