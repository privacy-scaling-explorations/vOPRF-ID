use std::env;
use std::str::FromStr;

use alloy::{
    network::EthereumWallet,
    primitives::{Address, FixedBytes, U256},
    providers::{Provider, ProviderBuilder},
    signers::local::PrivateKeySigner,
    sol,
};
use dotenv::dotenv;
use k256::{ProjectivePoint, Scalar};

use crate::utils::projective_to_ecpoint;

// Configuration struct to hold Ethereum settings
pub struct EthConfig {
    pub eth_rpc_url: String,
    pub registry_address: Address,
}

impl EthConfig {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        dotenv().ok();

        let eth_rpc_url = env::var("ETH_RPC_URL").expect("ETH_RPC_URL must be set in .env file");

        let registry_address = Address::from_str(
            &env::var("REGISTRY_ADDRESS").expect("REGISTRY_ADDRESS must be set in .env file"),
        )?;

        Ok(Self {
            eth_rpc_url,
            registry_address,
        })
    }
}

// Convert K256 ProjectivePoint to Registry contract's bytes32[2] format
pub fn point_to_bytes32_array(point: &ProjectivePoint) -> [FixedBytes<32>; 2] {
    let ec_point = projective_to_ecpoint(point);

    let x_bytes = hex::decode(&ec_point.x).unwrap();
    let y_bytes = hex::decode(&ec_point.y).unwrap();

    let mut x_fixed: [u8; 32] = [0; 32];
    let mut y_fixed: [u8; 32] = [0; 32];

    // Ensure we copy the right bytes (hex strings may have odd length)
    x_fixed[(32 - x_bytes.len())..].copy_from_slice(&x_bytes);
    y_fixed[(32 - y_bytes.len())..].copy_from_slice(&y_bytes);

    [FixedBytes::from(x_fixed), FixedBytes::from(y_fixed)]
}

// Register a node's public key in the Registry contract
pub async fn register_node(
    private_key: &Scalar,
    force: bool,
) -> Result<bool, Box<dyn std::error::Error>> {
    println!("Connecting to Ethereum...");

    // Get the appropriate config
    let config = EthConfig::new()?;

    // Calculate public key from private key
    let public_key = ProjectivePoint::GENERATOR * *private_key;
    println!("Public key calculated for registration");

    // For development purposes, we'll just log what we would do
    let public_key_bytes = point_to_bytes32_array(&public_key);
    println!(
        "Would register node with public key: [{:?}, {:?}]",
        public_key_bytes[0], public_key_bytes[1]
    );
    println!("Registry contract: {}", config.registry_address);

    if force {
        todo!("check if registered");
        todo!("submit transaction");
    }

    // For development, just return success
    println!("Successfully simulated node registration (not actually registered)");
    println!("To complete actual registration:");
    println!("1. Connect to the network at: {}", config.eth_rpc_url);
    println!(
        "2. Call register() on contract: {}",
        config.registry_address
    );
    println!(
        "3. With public key: [{:?}, {:?}]",
        public_key_bytes[0], public_key_bytes[1]
    );

    Ok(true)
}

// Check if the node is registered in the Registry contract
pub async fn check_node_registration(
    private_key: &Scalar,
) -> Result<bool, Box<dyn std::error::Error>> {
    println!("Connecting to Ethereum...");

    // Get the appropriate config
    let config = EthConfig::new()?;

    // Calculate public key from private key
    let public_key = ProjectivePoint::GENERATOR * *private_key;

    // For development purposes, we'll just log what we would do
    let public_key_bytes = point_to_bytes32_array(&public_key);
    println!(
        "Checking registration for public key: [{:?}, {:?}]",
        public_key_bytes[0], public_key_bytes[1]
    );
    println!("Registry contract: {}", config.registry_address);

    // For development, assume registered
    println!("Successfully simulated node registration check (assuming registered)");
    println!("To verify actual registration:");
    println!("1. Connect to the network at: {}", config.eth_rpc_url);
    println!(
        "2. Call isRegistered() on contract: {}",
        config.registry_address
    );
    println!(
        "3. Check if the public key matches: [{:?}, {:?}]",
        public_key_bytes[0], public_key_bytes[1]
    );

    Ok(true)
}
