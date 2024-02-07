use super::models::Course;
use super::state::AppState;
use actix_web::{web, HttpResponse};

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response= format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(response)
}

pub async fn get_course_for_tutor(app_state: web::Data<AppState>, params: web::Path<i32>) -> HttpResponse {
    HttpResponse::Ok().json("success")
}

pub async fn get_course_details(app_state: web::Data<AppState>, params: web::Path<(i32, i32)>) -> HttpResponse {
    HttpResponse::Ok().json("success")
}

pub async fn post_new_course(new_course: web::Json<Course>, app_state: web::Data<AppState>) -> HttpResponse {
    HttpResponse::Ok().json("success")
}
