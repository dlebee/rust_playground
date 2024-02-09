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

        // Spawn a task to handle user input
        let input_handler = tokio::spawn(handle_user_input());

        tokio::select! {
            Some(block) = stream1.next() => {
                println!("Creditcoin devnet - Ts: {:?}, block number: {} -> {:?}", block.timestamp, block.number.unwrap(), block.hash.unwrap());
               
            }
            Some(block) = stream2.next() => {
                println!("Creditcoin testnet - Ts: {:?}, block number: {} -> {:?}", block.timestamp, block.number.unwrap(), block.hash.unwrap());
            }
            input = input_handler => {
                match input {
                    Ok(user_input) => {
                        handle_user_command(user_input.unwrap());
                    }
                    Err(e) => eprintln!("Error reading user input: {}", e),
                }
            }
        }
    }
}

async fn handle_user_input() -> Result<String> {
    let mut reader = tokio::io::BufReader::new(stdin());
    let mut input = String::new();

    // Read user input line by line
    reader.read_line(&mut input).await?;

    Ok(input)
}

fn handle_user_command(command: String) {
    let sanitized_command = command.trim().to_lowercase();
    
    match sanitized_command.as_str() {
        "exit" => {
            println!("Exiting the program.");
            std::process::exit(0);
        }
        _ => {
            println!("Unknown command: {}", command);
        }
    }
}
