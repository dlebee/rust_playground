use ethers::providers::{Middleware, Provider, StreamExt, Ws};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let provider =
        Provider::<Ws>::connect("wss://rpc.cc3-devnet.creditcoin.network")
            .await?;
    let mut stream = provider.subscribe_blocks().await?;
    while let Some(block) = stream.next().await {
        println!(
            "Ts: {:?}, block number: {} -> {:?}",
            block.timestamp,
            block.number.unwrap(),
            block.hash.unwrap()
        );
    }

    Ok(())
}