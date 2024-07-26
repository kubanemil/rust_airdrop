use rust_airdrop::RPC_URL;
use solana_client::rpc_client::RpcClient;
use solana_program::{message::Message, pubkey::Pubkey, system_instruction::transfer};
use solana_sdk::{
    signature::{read_keypair_file, Signer},
    transaction::Transaction,
};
use std::str::FromStr;

fn main() {
    let client = RpcClient::new(RPC_URL);

    let kp = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
    let to_pubkey = Pubkey::from_str("FszSA5fufcJ5PVMppKTpM6BSpiag7uEGcfNwTt6PWerk").unwrap();

    let balance = client
        .get_balance(&kp.pubkey())
        .expect("Failed to get balance");
    let blockhash = client
        .get_latest_blockhash()
        .expect("Failed to get recent blockhash");
    let message = Message::new_with_blockhash(
        &[transfer(&kp.pubkey(), &to_pubkey, balance)],
        Some(&kp.pubkey()),
        &blockhash,
    );
    let fee = client
        .get_fee_for_message(&message)
        .expect("Failed to get fee calculator");

    let tx = Transaction::new_signed_with_payer(
        &[transfer(&kp.pubkey(), &to_pubkey, balance - fee)],
        Some(&kp.pubkey()),
        &vec![&kp],
        blockhash,
    );

    let sig = client
        .send_and_confirm_transaction(&tx)
        .expect("Failed to send transaction");

    println!(
        "Success! TX: https://explorer.solana.com/tx/{}/?cluster=devnet",
        sig
    );
}
