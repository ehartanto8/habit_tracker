use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub struct Habit {
    pub id: i64,
    pub description: String,
    pub done: bool,
}

#[derive(Deserialize)]
pub struct NewHabit {
    pub description: String
}