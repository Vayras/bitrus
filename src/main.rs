mod cli;
mod commands;
mod interactive;

use clap::Parser;
use cli::{Cli, Commands};
use commands::{fetch_block_stats, fetch_blocks, start_node};
use interactive::interactive_mode;

fn main() {
    let cli = Cli::parse();
    
    start_node();

    match cli.command {
        Some(Commands::FetchBlocks) => {
            if let Err(e) = fetch_blocks() {
                eprintln!("Error: {}", e);
            }
        }
        Some(Commands::BlockStats) => {
            if let Err(e) = fetch_block_stats() {
                eprintln!("Error: {}", e);
            }
        }
        None => interactive_mode(),
    }
}
