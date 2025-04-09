mod api;
mod cli;
mod eth_utils;
mod utils;

use clap::Parser;
use cli::{Cli, Commands};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Initialize { force } => {
            if let Err(e) = cli::handle_initialize(force).await {
                eprintln!("Initialization failed: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Serve => {
            // if let Err(e) = cli::check_private_key_exists().await {
            //     eprintln!("Error checking node status: {}", e);
            //     std::process::exit(1);
            // }
            println!("Starting server with private key: {:?}", utils::KEYS.0);
            api::run_server().await?;
        }
    }

    Ok(())
}
