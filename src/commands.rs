use bitcoincore_rpc::{Auth, Client, Error, RpcApi};
use std::process::Command;

pub fn create_rpc_client() -> Result<Client, Error> {
    let rpc_url = "http://127.0.0.1:18443";
    let rpc_auth = Auth::UserPass("bitcoinuser".into(), "strongpassword".into());
    Client::new(rpc_url, rpc_auth)
}


pub fn start_node() {
    let status = Command::new("bitcoind")
        .args(["-regtest", "-daemon"])
        .status()
        .expect("Bitcoin Core is stopping unexpectedly.");

    if status.success() {
        println!("bitcoind started successfully in regtest mode.");
    } else {
        eprintln!("Failed to start bitcoind. Check your configuration and bitcoind installation.");
    }
}

pub fn stop_node() {
    println!("Stopping Bitcoin Core...");
    match create_rpc_client() {
        Ok(rpc) => match rpc.stop() {
            Ok(msg) => println!("Node shutdown initiated: {}", msg),
            Err(e) => eprintln!("Failed to stop node via RPC: {}", e),
        },
        Err(e) => eprintln!("Failed to connect to Bitcoin RPC: {}", e),
    }
}

pub fn fetch_block_stats() -> Result<(), Box<dyn std::error::Error>> {
    let rpc = create_rpc_client()?;

    let info = rpc.get_blockchain_info()?;
    let height = info.blocks;

    let stats = rpc.get_block_stats(height)?;
    println!("Average TX Size: {:?}", stats.avg_tx_size);
    println!("Block Hash: {:?}", stats.block_hash);

    Ok(())
}

pub fn fetch_blocks() -> Result<(), Box<dyn std::error::Error>> {
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
        Err(_) => println!("Next block not available yet. Mine more blocks in regtest mode."),
    }

    Ok(())
}
