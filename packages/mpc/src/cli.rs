use std::fs;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use k256::{elliptic_curve::rand_core::OsRng, Scalar};

use crate::eth_utils;
use crate::utils::KEYS;

#[derive(Parser)]
#[command(author, version = "0.1.0", about = "vOPRF-ID MPC Node implementation", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize the MPC Node
    Initialize {
        /// Force reinitialize the MPC Node
        #[arg(short, long)]
        force: bool,
    },
    /// Start the MPC Node
    Serve,
}

pub async fn handle_initialize(force: bool) -> Result<(), Box<dyn std::error::Error>> {
    let private_key_path = PathBuf::from("./private_key.txt");

    if private_key_path.exists() && !force {
        println!("Private key file already exists. Use --force to overwrite.");
        return Ok(());
    }

    // Generate a new private key
    let private_key = Scalar::generate_vartime(&mut OsRng);

    // Register the node with the Registry contract
    if let Err(e) = eth_utils::register_node(&private_key, force).await {
        println!("Failed to register node: {}", e);
        return Err(e);
    }

    // Save the private key to file
    let bytes = private_key.to_bytes();
    fs::write(&private_key_path, bytes.as_slice())?;

    println!(
        "Successfully initialized private key at {}",
        private_key_path.display()
    );
    Ok(())
}

pub async fn check_private_key_exists() -> Result<(), Box<dyn std::error::Error>> {
    let private_key_path = PathBuf::from("./private_key.txt");

    if !private_key_path.exists() {
        println!("Private key file not found. Please run with 'initialize' command first.");
        std::process::exit(1);
    }

    // Check if node is registered in the Registry contract
    let is_registered = eth_utils::check_node_registration(&KEYS.0).await?;

    if !is_registered {
        println!("Node is not properly registered in the Registry contract.");
        println!(
            "Please run with 'initialize' command first, or check your Ethereum configuration."
        );
        std::process::exit(1);
    }

    Ok(())
}
