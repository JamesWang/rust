use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPool;
use std::env;
use std::io;
use std::sync::Mutex;

#[path ="../iter2/handlers.rs"]
mod handlers;

#[path = "../iter2/models.rs"]
mod models;

#[path = "../iter2/routes.rs"]
mod routes;

#[path = "../iter2/state.rs"]
mod state;

use routes::*;
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
    HttpServer::new(app).bind("0.0.0.0:3000")?.run().await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use chrono::NaiveDate;
    use dotenv::dotenv;
    use sqlx::postgres::PgPool;
    use std::env;
    use std::sync::Mutex;
    use crate::handlers::get_course_for_tutor;

    #[actix_rt::test]
    async fn get_all_courses_success(){
        dotenv().ok();
        let database_rul = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_rul).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState{
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        let tutor_id: web::Path<i32> = web::Path::from(1);
        let resp = get_course_for_tutor(app_state, tutor_id).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}