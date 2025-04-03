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
            cli::handle_initialize(force)?;
        }
        Commands::Serve => {
            cli::check_private_key_exists()?;
            println!("Starting server with private key: {:?}", utils::KEYS.0);
            api::run_server().await?;
        }
    }

    Ok(())
}
