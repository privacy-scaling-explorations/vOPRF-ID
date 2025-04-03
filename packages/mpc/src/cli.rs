use std::fs;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use k256::{elliptic_curve::rand_core::OsRng, Scalar};

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

pub fn handle_initialize(force: bool) -> std::io::Result<()> {
    let private_key_path = PathBuf::from("./private_key.txt");

    if private_key_path.exists() && !force {
        println!("Private key file already exists. Use --force to overwrite.");
        return Ok(());
    }

    let private_key = Scalar::generate_vartime(&mut OsRng);
    let bytes = private_key.to_bytes();
    fs::write(&private_key_path, bytes.as_slice())?;

    println!(
        "Successfully initialized private key at {}",
        private_key_path.display()
    );
    Ok(())
}

pub fn check_private_key_exists() -> std::io::Result<()> {
    let private_key_path = PathBuf::from("./private_key.txt");

    if !private_key_path.exists() {
        println!("Private key file not found. Please run with 'initialize' command first.");
        std::process::exit(1);
    }

    Ok(())
}
