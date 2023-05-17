use rust_tutorial::prereq;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{commitment_config::CommitmentConfig, signer::Signer};

use rust_tutorial::DEVNET;
use rust_tutorial::{check_balance, get_wallet_keypair};
// use rust_tutorial::LOCALHOST;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // DON'T FORGET TO CHANGE URL AND CLUSTER between devnet and custom which is localhost.
    let cluster = "cluster=devnet";
    // let cluster = "cluster=custom";
    let client: RpcClient = RpcClient::new_with_commitment(DEVNET, CommitmentConfig::confirmed());

    let wallet_keypair = get_wallet_keypair()?;
    let my_keypair = prereq::keygen();
    match prereq::airdop(&client, &my_keypair) {
        Ok(sig) => {
            println!("Success! Check out your TX here:");
            println!("https://explorer.solana.com/tx/{}?{}\n\n", sig, cluster);

            if let Ok(bal) = check_balance(&client, &my_keypair.pubkey()) {
                println!("Pubkey: {}, has balance: {bal}", my_keypair.pubkey());
            }

            let sig = prereq::transfer_sol(&client, &wallet_keypair, &my_keypair);
            if sig.is_ok() {
                println!(
                    "Success! Transfer SOL.\nCheck out your TX here: 
                    https://explorer.solana.com/tx/{}/?{}\n\n",
                    sig.unwrap(),
                    cluster
                );
            } else {
                println!("Error transfer_sol");
            }

            let sig = prereq::empty(&client, wallet_keypair, my_keypair);
            if sig.is_ok() {
                println!(
                    "Success! Empty walet.\nCheck out your TX here: 
                    https://explorer.solana.com/tx/{}/?{}\n\n",
                    sig.unwrap(),
                    cluster
                );
            } else {
                println!("Error empty wallet.");
            }

            Ok(())
        }
        Err(e) => Err(e),
    }
}
