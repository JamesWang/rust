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
    use crate::handlers::{get_course_details, get_course_for_tutor, post_new_course};
    async fn get_app_state() -> web::Data<AppState> {
        let database_rul = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_rul).await.unwrap();
        web::Data::new(AppState{
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        })
    }
    #[actix_rt::test]
    async fn get_all_courses_success(){
        dotenv().ok();
        let app_state: web::Data<AppState> = get_app_state().await;
        let tutor_id: web::Path<i32> = web::Path::from(1);
        let resp = get_course_for_tutor(app_state, tutor_id).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_course_detail_test() {
        dotenv().ok();
        let app_state: web::Data<AppState> = get_app_state().await;
        let params: web::Path<(i32, i32)> = web::Path::from((1,2));
        let resp = get_course_details(app_state, params).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    use models::Course;
    #[actix_rt::test]
    async fn post_course_success() {
        dotenv().ok();
        let app_state: web::Data<AppState> = get_app_state().await;
        let new_course_msg = Course {
            course_id: 1,
            tutor_id: 1,
            course_name: "This is the next course".into(),
            posted_time: Some(NaiveDate::from_ymd(2020, 9, 17).and_hms(
                14, 01, 11)),
        };
        let course_param = web::Json(new_course_msg);
        let resp = post_new_course(course_param, app_state).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}