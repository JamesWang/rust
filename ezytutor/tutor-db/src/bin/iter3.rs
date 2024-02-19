use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPool;
use std::env;
use std::io;
use std::sync::Mutex;
use crate::routes::{course_routes, general_routes};
use crate::state::AppState;

#[path ="../iter3/handlers.rs"]
mod handlers;

#[path = "../iter3/models.rs"]
mod models;

#[path = "../iter3/routes.rs"]
mod routes;

#[path = "../iter3/state.rs"]
mod state;

#[path = "../iter3/db_access.rs"]
mod db_access;

#[actix_rt::main]
async fn main() -> io::Result<()>{
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = PgPool::connect(&db_url).await.unwrap();
    let shared_data = web::Data::new(AppState{
        health_check_response: "I'm good. you've already asked me".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool
    });

    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(general_routes)
            .configure(course_routes)
    };
    HttpServer::new(app).bind("0.0.0.0:3000")?.run().await
}