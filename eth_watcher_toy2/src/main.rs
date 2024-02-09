use ethers::providers::{Middleware, Provider, StreamExt, Ws};
use eyre::Result;
use tokio::io::{self, AsyncBufReadExt, stdin};

#[tokio::main]
async fn main() -> Result<()> {
    // Connect to the first blockchain
    let provider1 = Provider::<Ws>::connect("wss://rpc.cc3-devnet.creditcoin.network").await?;
    let mut stream1 = provider1.subscribe_blocks().await?;

    // Connect to the second blockchain
    let provider2 = Provider::<Ws>::connect("wss://rpc.cc3-testnet.creditcoin.network").await?;
    let mut stream2 = provider2.subscribe_blocks().await?;


    loop {

        tokio::select! {
            Some(block) = stream1.next() => {
                println!("Creditcoin devnet - Ts: {:?}, block number: {} -> {:?}", block.timestamp, block.number.unwrap(), block.hash.unwrap());
               
            }
            Some(block) = stream2.next() => {
                println!("Creditcoin testnet - Ts: {:?}, block number: {} -> {:?}", block.timestamp, block.number.unwrap(), block.hash.unwrap());
            }
        }
    }
}