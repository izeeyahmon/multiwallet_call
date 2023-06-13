use ethers::prelude::*;
use ethers::providers::{Http, Provider};

pub async fn send_transaction(
    wallet: LocalWallet,
    provider: Provider<Http>,
    contract_address: H160,
    transaction_data: Bytes,
    transaction_value: U256,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = SignerMiddleware::new(provider.clone(), wallet.clone());

    let transaction_request = TransactionRequest::new()
        .to(contract_address)
        .data(transaction_data)
        .value(transaction_value);
    let tx = client.send_transaction(transaction_request, None).await?.await?;
    println!("Transaction Receipt: {:?}", &tx);

    Ok(())
}
