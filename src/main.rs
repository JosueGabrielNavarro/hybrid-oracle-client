// Importing Libraries
use std::{
    env,
    str::FromStr,
};
use alloy::{
    primitives::Address,
    providers::ProviderBuilder,
    sol,
};
use dotenv::dotenv;
use eyre::Result;

// Importing the interface
sol! {
    #[sol(rpc)]
    interface PriceConsumer {
        function getLatestPrice() external view returns (int256 price);
    }
}

// Tokio
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    // Importing credentials
    dotenv().ok();

    // Reading credentials
    let rpc_url = env::var("RPC_URL")?.parse()?;
    let contract_address = env::var("CONTRACT_ADDRESS")?.parse()?;

    // Initialize our provider
    let provider = ProviderBuilder::new().on_http(rpc_url);
    let contract = PriceConsumer::new(contract_address, provider);

    // Calling getLatestPrice
    let price_raw = contract.getLatestPrice().call().await?;

    // Formating price
    let price_formatted = price_raw.price.to_string().parse::<f64>()? / 100_000_000.0;

    // printing the price
    println!("-----------------------------------------------------------------------");
    println!("Connection established");
    println!("Current Price: {:.2}", price_formatted);
    println!("-----------------------------------------------------------------------");
    
    Ok(())
}