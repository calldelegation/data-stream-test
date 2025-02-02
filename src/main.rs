use fuel_streams::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client with custom connection options
    let mut client = Client::new(FuelNetwork::Mainnet).with_api_key("fuel-api-key");
    Ok(())
}