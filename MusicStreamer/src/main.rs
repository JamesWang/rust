mod http;

use std::sync::Arc;
use http::admin;
use crate::admin::MusicConf;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let music_path = "V:\\MusicPhotos\\music".to_string();
    admin::start_server_at("localhost:8088".to_string(), music_path).await
}
