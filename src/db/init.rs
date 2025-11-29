use sqlx::{Pool, Sqlite};

pub async fn init_db(pool: &Pool<Sqlite>) {
    sqlx::query(
        r#"
            CREATE TABLE IF NOT EXISTS habits (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                description TEXT NOT NULL,
                done BOOLEAN NOT NULL DEFAULT 0
            );
            "#
    )
        .execute(pool)
        .await
        .expect("Failed to initialized database");
}