use crate::LAMPORTS_PER_SOL;
use solana_client::rpc_client::RpcClient;
use solana_program::system_instruction::transfer;
use solana_sdk::{
    message::Message,
    signature::{Keypair, Signature as Sgnt, Signer},
    transaction::Transaction,
};

pub fn keygen() -> Keypair {
    let kp = Keypair::new();
    let mut res = String::new();
    res.push_str("You've generated a new Solana wallet with public key: ");
    res.push_str(kp.pubkey().to_string().as_str());
    print!("{res}\n\n");
    kp
}

pub fn airdop(client: &RpcClient, key: &Keypair) -> Result<Sgnt, Box<dyn std::error::Error>> {
    println!("Request airdrop for {}", key.pubkey());
    let sig = client.request_airdrop(&key.pubkey(), (0.5 * LAMPORTS_PER_SOL) as u64)?;
    loop {
        let confirmed = client.confirm_transaction(&sig)?;
        if confirmed {
            break;
        } else {
            std::thread::sleep(std::time::Duration::from_millis(80));
            println!("Request airdrop again..");
        }
    }
    Ok(sig)
}

pub fn transfer_sol(
    client: &RpcClient,
    to: &Keypair,
    from: &Keypair,
) -> Result<Sgnt, Box<dyn std::error::Error>> {
    let recent_blockhash = client.get_latest_blockhash()?;
    let transaction = Transaction::new_signed_with_payer(
        &[transfer(&from.pubkey(), &to.pubkey(), 100)],
        Some(&from.pubkey()),
        &vec![from],
        recent_blockhash,
    );

    let signature = client.send_and_confirm_transaction(&transaction)?;
    Ok(signature)
}

pub fn empty(
    client: &RpcClient,
    to: Keypair,
    from_keypair: Keypair,
) -> Result<Sgnt, Box<dyn std::error::Error>> {
    let from = from_keypair.pubkey();
    let balance = client.get_balance(&from)?;

    let recent_blockhash = client.get_latest_blockhash()?;
    let message = Message::new_with_blockhash(
        &[transfer(&from, &to.pubkey(), balance)],
        Some(&from),
        &recent_blockhash,
    );

    let fee = client.get_fee_for_message(&message)?;
    let transaction = Transaction::new_signed_with_payer(
        &[transfer(&from, &to.pubkey(), balance - fee)],
        Some(&from),
        &vec![&from_keypair],
        recent_blockhash,
    );
    let signature = client.send_and_confirm_transaction(&transaction)?;
    Ok(signature)
}
