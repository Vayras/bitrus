use crate::commands::{fetch_blocks, fetch_block_stats, stop_node};
use std::io::{self, Write};

fn display_help() {
    println!("\nAvailable commands:");
    println!("  fetchblocks - Display recent blocks information");
    println!("  blockstats  - Display block statistics");
    println!("  help        - Show this help message");
    println!("  exit        - Stop node and exit application");
}

pub fn interactive_mode() {
    println!("Bitcoin CLI Interactive Mode");
    println!("Type 'help' for commands or 'exit' to quit\n");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim().to_lowercase().as_str() {
            "fetchblocks" => {
                if let Err(e) = fetch_blocks() {
                    eprintln!("Error: {}", e);
                }
            }
            "blockstats" => {
                if let Err(e) = fetch_block_stats() {
                    eprintln!("Error: {}", e);
                }
            }
            "help" => display_help(),
            "exit" | "quit" => {
                stop_node();
                println!("Exiting...");
                break;
            }
            "" => continue,
            unknown => println!("Unknown command: '{}'. Type 'help' for a list of commands.", unknown),
        }
    }
}
