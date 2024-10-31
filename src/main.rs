use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::Signature;
use solana_transaction_status::UiTransactionEncoding;
use std::str::FromStr;

fn main() -> Result<()> {
    // Initialize RPC client with Solana devnet URL
    let rpc_url = "https://api.devnet.solana.com".to_string();
    let client = RpcClient::new(rpc_url);

    // Example transaction signature (you can replace this with command line args later)
    let tx_signature =
        "4uoM6e5Nxiwj4c5smoV3eXkNDbBqfNRmCiTrpkCzq2wQ3G2J2amdjjkXgVCmHfXPU2hXhapNg8Mixoar4R7Mni8g";

    // Parse the signature
    let signature = Signature::from_str(tx_signature)?;

    // Get transaction details
    match client.get_transaction(&signature, UiTransactionEncoding::Base64) {
        Ok(tx) => {
            println!("Transaction details: {:#?}", tx);
            Ok(())
        }
        Err(e) => {
            println!("Error fetching transaction: {}", e);
            Err(e.into())
        }
    }
}
