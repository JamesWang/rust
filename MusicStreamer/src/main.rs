mod http;

use http::admin;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    admin::start_server_at("localhost:8088".to_string()).await
}
