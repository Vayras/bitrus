use clap::{Parser, Subcommand};
use std::process::Command;
use bitcoincore_rpc::{Auth, Client, RpcApi};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli{
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]

enum Commands {
    StartNode,
    FetchBlocks,
    BlockSats,
}

fn start_node() {
    let status = Command::new("bitcoind")
        .args(["-regtest","-daemon"])
        .status()
        .expect("failed to execute bitcoind");
    
    if status.success(){
        eprintln!("bitcoind started successfully in regtest mode.");
    } else {
        eprintln!("Failed to starr bitcoind. Check your configuration and bitcoind installation.");
    }
}

fn create_rpc_client() -> Result<Client, bitcoincore_rpc::Error> {
    let rpc_url = "http://127.0.0.1:18443";
    let rpc_auth = Auth::UserPass("bitcoinuser".to_string(), "strongpassword".to_string());
    Client::new(rpc_url, rpc_auth)
}

fn fetch_blocks_stats() -> Result<(), Box<dyn std::error::Error>> {
    let rpc = create_rpc_client()?;
    
    let info = rpc.get_blockchain_info()?;
    let height = info.blocks;

    let block_stats = rpc.get_block_stats(height)?;
    println!("avg_tx_size : {:?}", block_stats.avg_tx_size);
    println!("Block Hash : {:?}", block_stats.block_hash);
    
    Ok(())
}

fn fetch_blocks() -> Result<(), Box<dyn std::error::Error>> {
    let rpc = create_rpc_client()?;
    
    let info = rpc.get_blockchain_info()?;
    let height = info.blocks;

    let latest_hash = rpc.get_block_hash(height)?;
    println!("Latest Block Hash: {}", latest_hash);
    
    
    if height > 0 {
        let previous_hash = rpc.get_block_hash(height - 1)?;
        println!("Previous Block Hash: {}", previous_hash);
    } else {
        println!("At genesis block; no previous block available.");
    }
    
    match rpc.get_block_hash(height + 1) {
        Ok(next_hash) => println!("Next Block Hash: {}", next_hash),
        Err(_) => println!("Next block not available yet. You may need to mine additional blocks in regtest mode."),
    }
    
    Ok(())
}

fn main(){
    let cli = Cli::parse();

    match cli.command {
        Commands::StartNode => start_node(),
        Commands::FetchBlocks => {
            if let  Err(e) = fetch_blocks(){
                eprintln!("Error; {}", e)
            }
        }
        Commands::BlockSats => {
            if let  Err(e) = fetch_blocks_stats(){
                eprintln!("Error; {}", e)
            }
        }
    }
}