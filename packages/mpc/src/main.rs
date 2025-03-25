mod api;
mod utils;

use utils::KEYS;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server with private key: {:?}", KEYS.0);
    api::run_server().await
}
