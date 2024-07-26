use rust_airdrop::{CompleteArgs, WbaPrereqProgram, RPC_URL};
use solana_client::rpc_client::RpcClient;
use solana_program::system_program;
use solana_sdk::signature::{read_keypair_file, Signer};

fn main() {
    let rpc_client = RpcClient::new(RPC_URL);

    let wba_kp = read_keypair_file("wba-wallet.json").expect("Couldn't find wallet file");
    let prereq =
        WbaPrereqProgram::derive_program_address(&[b"prereq", wba_kp.pubkey().to_bytes().as_ref()]);
    let args = CompleteArgs {
        github: b"kubanemil".to_vec(),
    };
    let blockhash = rpc_client
        .get_latest_blockhash()
        .expect("Failed to get recent blockhash");

    let tx = WbaPrereqProgram::complete(
        &[&wba_kp.pubkey(), &prereq, &system_program::id()],
        &args,
        Some(&wba_kp.pubkey()),
        &[&wba_kp],
        blockhash,
    );

    let sig = rpc_client
        .send_and_confirm_transaction(&tx)
        .expect("Failed to send transaction");

    println!(
        "Success! TX: https://explorer.solana.com/tx/{}/?cluster=devnet",
        sig
    );
}
