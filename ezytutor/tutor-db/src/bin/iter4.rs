use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use routes::{general_routes, course_routes};
use sqlx::postgres::PgPool;
use std::env;
use std::io;
use std::sync::Mutex;

#[path ="../iter4/handlers.rs"]
mod handlers;

#[path = "../iter4/models.rs"]
mod models;

#[path = "../iter4/routes.rs"]
mod routes;

#[path = "../iter4/state.rs"]
mod state;

#[path = "../iter4/db_access.rs"]
mod db_access;

#[path ="../iter4/errors.rs"]
mod errors;

use state::AppState;

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
    let host_port = env::var("HOST_PORT").expect("HOST:PORT address is not set in .env file");

    HttpServer::new(app).bind(host_port)?.run().await
}