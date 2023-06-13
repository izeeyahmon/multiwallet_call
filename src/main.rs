mod errors;
mod readcsv;
mod transaction;

use crate::errors::print_help;
use dotenv::dotenv;
use ethers::prelude::*;
use ethers::types::H256;
use readcsv::read_csv_from_path;
use std::env;
use transaction::send_transaction;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    dotenv().ok();
    if args.len() == 3 {
        let private_keys = read_csv_from_path(&args[1]);
        println!("Private keys loaded: {:?}", &private_keys.len());
        let rpc_url = std::env::var("RPC").expect("Need an RPC to start");
        let provider = Provider::<Http>::try_from(&rpc_url)?;
        let tx_hash_details = provider
            .get_transaction(args[2].clone().parse::<H256>().unwrap())
            .await?
            .expect("Need a Valid TX Hash to QuickTask");

        let mut handles = Vec::new();
        for sk in private_keys.into_iter() {
            let wallet: LocalWallet = sk.parse::<LocalWallet>()?.with_chain_id(
                std::env::var("CHAIN_ID")
                    .expect("Need Chain ID")
                    .parse::<u64>()
                    .unwrap(),
            );
            let provider = Provider::<Http>::try_from(&rpc_url)?;
            let contract = tx_hash_details.to.unwrap();
            let transaction_data = tx_hash_details.input.clone();
            let value = tx_hash_details.value;
            let handle = tokio::spawn(async move {
                send_transaction(
                    wallet.clone(),
                    provider.clone(),
                    contract,
                    transaction_data,
                    value,
                )
                .await
                .ok();
            });

            handles.push(handle);
        }
        futures::future::join_all(handles).await;
        Ok(())
    } else {
        print_help();
        panic!("No command line arguments provided or too many args!");
    }
}
