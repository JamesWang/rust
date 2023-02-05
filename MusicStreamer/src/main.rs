mod http;
mod jokebox;
mod server;

use std::sync::Arc;
use actix_rt::spawn;
use actix_web::{App, HttpServer, web};
use http::admin;
use crate::admin::MusicConf;
use crate::server::tcp_server::IncomingHandler;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let music_path = "V:\\MusicPhotos\\music".to_string();
    let m2 = music_path.clone();
    //admin::start_server_at(&String::from("localhost:8088"), &music_path).await
    std::thread::spawn(move||{
        server::tcp_server::listen_on(
            3300,
            server::music_commands::MusicInfo::new(m2)
        );
    });
    admin::start_server_at2("localhost:8088", music_path).await

}
