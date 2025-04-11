use bitcoincore_rpc::{RpcApi};
use std::process::Command;
use crate::rpc_client::create_rpc_client;

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

pub fn fetch_block_chain_data() -> Result<(), Box<dyn std::error::Error>> {
    let rpc = create_rpc_client()?;

    let info = rpc.get_blockchain_info()?;

    println!("chain  : {:?}", info.chain);
    println!("height : {:?}", info.blocks);
    println!("headers : {:?}", info.headers);
    println!("best_block_hash : {:?}", info.best_block_hash);
    println!("difficulty: {:?}", info.difficulty);

    let txoutset_info = rpc.get_tx_out_set_info(None, None, None)?;

    println!("utx_outs  : {:?}", txoutset_info.tx_outs);
    println!("transactions : {:?}", txoutset_info.transactions);
    println!("total_amount  : {:?}", txoutset_info.total_amount);
    println!("transactions : {:?}", txoutset_info.transactions);

    Ok(())
}
