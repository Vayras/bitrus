Simple command-line interface written in Rust for managing Bitcoin Core (`bitcoind`) in **regtest mode** and interfacing with its **JSON-RPC API**.

This tool helps developers interact with a local Bitcoin node easily—fetching block hashes, inspecting block stats, and exploring regtest mode with both scripted and interactive modes.

---

## Features

- Starts and stops a local `bitcoind` instance in regtest mode
- Fetch latest, previous, and next block hashes
- View basic block statistics via `getblockstats`
- Interactive CLI mode and command-line argument support
- Built in Rust using `clap` and `bitcoincore-rpc` crates
- more Features are being added daily.

---

## Getting Started

### 1. Prerequisites

- [Rust](https://rustup.rs/) installed (`cargo` and `rustc`)
- [Bitcoin Core](https://bitcoincore.org/en/download/) installed (`bitcoind`, `bitcoin-cli`)
- Optional: [cargo-watch](https://github.com/watchexec/cargo-watch) for development

### 2. Clone & Build

```bash
git clone https://github.com/yourusername/bitcoin-cli-rust.git
cd bitcoin-cli-rust
cargo build --release
