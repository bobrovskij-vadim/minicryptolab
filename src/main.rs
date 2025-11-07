mod hash;
mod blockchain;

use clap::{Parser, Subcommand};
use std::fs;

#[derive(Parser)]
#[command(name = "MiniCryptoLab")]
#[command(about = "Rust Crypto Lab", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Hash { text: String },       // Calculate SHA256 hash
    History,                     // Show hash history
    AddBlock { data: String },   // Add new block
    ShowChain,                   // Show blockchain
    ValidateChain,               // Validate blockchain integrity
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Hash { text } => hash::run(text),
        Commands::History => show_history(),
        Commands::AddBlock { data } => blockchain::add_block(data),
        Commands::ShowChain => blockchain::show_chain(),
        Commands::ValidateChain => blockchain::validate_chain(),
    }
}

fn show_history() {
    let file = "hash_history.json";
    match fs::read_to_string(file) {
        Ok(data) => println!("📜 Story:\n{}", data),
        Err(_) => println!("❌ The history is empty for now. Run the 'hash' command first."),
    }
}