pub mod prereq;
pub mod programs;

use solana_client::rpc_client::RpcClient;
use solana_sdk::{
        signer::keypair::read_keypair_file,
        signature::Keypair,
        pubkey::Pubkey,
};

pub const LAMPORTS_PER_SOL: f64 = 1_000_000_000.0;
pub const DEVNET: &str = "https://api.devnet.solana.com";
pub const LOCALHOST: &str = "http://localhost:8899";

pub fn get_wallet_keypair()-> Result<Keypair, Box<dyn std::error::Error>> {
    read_keypair_file("wba-wallet.json")
}

pub fn check_balance(client: &RpcClient, public_key: &Pubkey) -> Result<f64, Box<dyn std::error::Error>> {
    Ok(client.get_balance(public_key)? as f64 / LAMPORTS_PER_SOL)
}