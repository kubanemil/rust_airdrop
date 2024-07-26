use rust_airdrop::RPC_URL;
use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::{read_keypair_file, Signer};

fn main() {
    let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
    println!("Public Key: {}", keypair.pubkey());
    let client = RpcClient::new(RPC_URL);
    match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
        Ok(s) => {
            println!("Success! Check out your TX here:");
            println!("https://explorer.solana.com/tx/{}?cluster=devnet", s);
        }
        Err(e) => println!("Oops, something went wrong: {}", e),
    };
}
