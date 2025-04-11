use crate::commands::{fetch_block_chain_data, fetch_block_stats, stop_node, get_network_info};
use std::io::{self, Write};

fn display_help() {
    println!("\nAvailable commands:");
    println!("  blockchaindata - Display recent blocks information");
    println!("  blockstats  - Display block statistics");
    println!("  networkinfo  - Display block statistics");
    println!("  help        - Show this help message");
    println!("  exit        - Stop node and exit application");
    println!("");
}

pub fn interactive_mode() {

    println!("Bitcoin CLI Interactive Mode");
    display_help();
    
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim().to_lowercase().as_str() {
            "blockchaindata" => {
                if let Err(e) = fetch_block_chain_data() {
                    eprintln!("Error: {}", e);
                }
            }
            "networkinfo" => {
                if let Err(e) = get_network_info() {
                    eprintln!("Error: {}", e);
                }
            }
            "blockstats" => {
                if let Err(e) = fetch_block_stats() {
                    eprintln!("Error: {}", e);
                }
            }
            "help" => display_help(),
            "exit" | "quit" | "clear" => {
                stop_node();
                println!("Exiting...");
                break;
            }
            "" => continue,
            unknown => println!("Unknown command: '{}'. Type 'help' for a list of commands.", unknown),
        }
    }
}
