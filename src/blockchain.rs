use sha2::{Digest, Sha256};
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;
use chrono::Utc;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    index: u64,
    timestamp: String,
    data: String,
    prev_hash: String,
    hash: String,
}

const CHAIN_FILE: &str = "blockchain.json";

impl Block {
    // Create a new block with given data and previous hash
    pub fn new(index: u64, data: String, prev_hash: String) -> Self {
        let timestamp = Utc::now().to_rfc3339();

        // Combine all fields into a single string for hashing
        let record = format!("{}{}{}{}", index, timestamp, data, prev_hash);

        // Calculate SHA256 hash of the block
        let mut hasher = Sha256::new();
        hasher.update(record.as_bytes());
        let hash = format!("{:x}", hasher.finalize());

        Block {
            index,
            timestamp,
            data,
            prev_hash,
            hash,
        }
    }

    // Recalculate the hash from block data
    fn calculate_hash(&self) -> String {
        let record = format!("{}{}{}{}", self.index, self.timestamp, self.data, self.prev_hash);
        let mut hasher = Sha256::new();
        hasher.update(record.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

pub fn add_block(data: String) {
    // Load existing blockchain from file
    let mut chain: Vec<Block> = if Path::new(CHAIN_FILE).exists() {
        let content = fs::read_to_string(CHAIN_FILE).unwrap_or_default();
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        Vec::new()
    };

    // Get the previous hash or "GENESIS" if it's the first block
    let prev_hash = if let Some(last_block) = chain.last() {
        last_block.hash.clone()
    } else {
        String::from("GENESIS")
    };

    // Create a new block
    let block = Block::new(chain.len() as u64, data, prev_hash);

    // Append to blockchain
    chain.push(block);

    // Save the updated chain
    let json = serde_json::to_string_pretty(&chain).unwrap();
    fs::write(CHAIN_FILE, json).expect("Failed to write blockchain");

    println!("✅ Block added successfully!");
}

pub fn show_chain() {
    if let Ok(data) = fs::read_to_string(CHAIN_FILE) {
        println!("📦 Blockchain:\n{}", data);
    } else {
        println!("❌ No blockchain found. Add a block first!");
    }
}

pub fn validate_chain() {
    // Load blockchain
    let content = fs::read_to_string(CHAIN_FILE);
    if content.is_err() {
        println!("❌ Blockchain file not found!");
        return;
    }

    let chain: Vec<Block> = serde_json::from_str(&content.unwrap()).unwrap_or_default();

    if chain.is_empty() {
        println!("❌ Blockchain is empty!");
        return;
    }

    // Validate each block
    for i in 0..chain.len() {
        let block = &chain[i];

        // 1. Verify current block hash
        let recalculated = block.calculate_hash();
        if block.hash != recalculated {
            println!("❌ Invalid hash at block {}", block.index);
            return;
        }

        // 2. Verify previous hash link (skip genesis block)
        if i > 0 {
            let prev = &chain[i - 1];
            if block.prev_hash != prev.hash {
                println!("❌ Broken chain at block {}", block.index);
                return;
            }
        }
    }

    println!("✅ Blockchain integrity verified. All blocks are valid!");
}