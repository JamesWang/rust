mod http;

use std::sync::Arc;
use actix_web::{App, HttpServer, web};
use http::admin;
use crate::admin::MusicConf;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let music_path = "V:\\MusicPhotos\\music".to_string();
    //admin::start_server_at(&String::from("localhost:8088"), &music_path).await
    admin::start_server_at2(&String::from("localhost:8088"), music_path).await
}
