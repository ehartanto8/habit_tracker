mod db;
mod services;
mod routes;
mod models;

use actix_web::{App, HttpServer, web};
use dotenvy::dotenv;
use std::env;
use sqlx::{Pool, Sqlite};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = Pool::<Sqlite>::connect(&database_url)
        .await
        .expect("Failed to create pool");

    db::init::init_db(&pool).await;

    println!("Starting server on http://localhost: 3000");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::resource("/habits")
                    .route(web::get().to(routes::habits::placeholder))
            )
    })
        .bind(("127.0.0.1", 3000)) ?
        .run()
        .await
}
