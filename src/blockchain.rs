use sha2::{Digest, Sha256};
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;
use chrono::Utc;
use p256::ecdsa::{signature::Signer, signature::Verifier, Signature, SigningKey, VerifyingKey};
use crate::keys;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    index: u64,
    timestamp: String,
    data: String,
    prev_hash: String,
    hash: String,
    signature: Option<String>,
    public_key: Option<String>,
    nonce: u64, // Proof of Work field
}

const CHAIN_FILE: &str = "blockchain.json";

impl Block {
    // Create a new block and mine it according to difficulty
    pub fn new(index: u64, data: String, prev_hash: String, difficulty: usize) -> Self {
        let timestamp = Utc::now().to_rfc3339();
        let mut nonce = 0u64;
        let target = "0".repeat(difficulty);
        let mut hash;

        // Proof of Work loop
        loop {
            let record = format!("{}{}{}{}{}", index, timestamp, data, prev_hash, nonce);
            let mut hasher = Sha256::new();
            hasher.update(record.as_bytes());
            hash = format!("{:x}", hasher.finalize());

            if nonce % 10000 == 0{
                println!("⛏️ Mining... nonce = {}", nonce);
            }

            if hash.starts_with(&target) {
                break;
            }
            nonce += 1;
        }

        Block {
            index,
            timestamp,
            data,
            prev_hash,
            hash,
            signature: None,
            public_key: None,
            nonce,
        }
    }

    // Calculate hash from raw string
    fn calculate_hash_raw(record: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(record.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    // Recalculate hash from block fields
    fn calculate_hash(&self) -> String {
        let record = format!("{}{}{}{}{}", self.index, self.timestamp, self.data, self.prev_hash, self.nonce);
        Self::calculate_hash_raw(&record)
    }

    // Sign the block using private key
    pub fn sign(&mut self, key: &SigningKey, pubkey: &VerifyingKey) {
        let signature: Signature = key.sign(self.hash.as_bytes());
        self.signature = Some(hex::encode(signature.to_der()));
        self.public_key = Some(hex::encode(pubkey.to_encoded_point(false).as_bytes()));
    }

    // Verify block signature
    pub fn verify_signature(&self) -> bool {
        if let (Some(sig_hex), Some(pub_hex)) = (&self.signature, &self.public_key) {
            let sig_bytes = match hex::decode(sig_hex) {
                Ok(v) => v,
                Err(_) => return false,
            };
            let pub_bytes = match hex::decode(pub_hex) {
                Ok(v) => v,
                Err(_) => return false,
            };

            let signature = match Signature::from_der(&sig_bytes) {
                Ok(v) => v,
                Err(_) => return false,
            };
            let point = match p256::EncodedPoint::from_bytes(&pub_bytes) {
                Ok(p) => p,
                Err(_) => return false,
            };
            let pubkey = match VerifyingKey::from_encoded_point(&point) {
                Ok(k) => k,
                Err(_) => return false,
            };

            pubkey.verify(self.hash.as_bytes(), &signature).is_ok()
        } else {
            false
        }
    }
}

// Add new block with mining difficulty
pub fn add_block(data: String, difficulty: usize) {
    let mut chain: Vec<Block> = if Path::new(CHAIN_FILE).exists() {
        let content = fs::read_to_string(CHAIN_FILE).unwrap_or_default();
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        Vec::new()
    };

    let prev_hash = if let Some(last_block) = chain.last() {
        last_block.hash.clone()
    } else {
        String::from("GENESIS")
    };

    println!("⛏️ Mining new block with difficulty {}...", difficulty);

    let mut block = Block::new(chain.len() as u64, data, prev_hash, difficulty);

    if let (Some(priv_key), Some(pub_key)) = (keys::load_private_key(), keys::load_public_key()) {
        block.sign(&priv_key, &pub_key);
        println!("🔏 Block signed successfully!");
    } else {
        println!("⚠️ No keys found. Run 'generate-keys' first.");
    }

    chain.push(block);
    let json = serde_json::to_string_pretty(&chain).unwrap();
    fs::write(CHAIN_FILE, json).expect("Failed to write blockchain");
    println!("✅ Block added to blockchain!");
}

// Validate all signatures
pub fn validate_signatures() {
    let content = fs::read_to_string(CHAIN_FILE);
    if content.is_err() {
        println!("❌ Blockchain not found!");
        return;
    }

    let chain: Vec<Block> = serde_json::from_str(&content.unwrap()).unwrap_or_default();
    if chain.is_empty() {
        println!("❌ Blockchain is empty!");
        return;
    }

    for block in &chain {
        if block.signature.is_none() {
            println!("⚠️ Block {} is unsigned.", block.index);
            continue;
        }

        if block.verify_signature() {
            println!("✅ Block {} signature is valid.", block.index);
        } else {
            println!("❌ Block {} signature is INVALID!", block.index);
        }
    }
}

// Display blockchain contents
pub fn show_chain() {
    if let Ok(data) = fs::read_to_string(CHAIN_FILE) {
        println!("📦 Blockchain:\n{}", data);
    } else {
        println!("❌ No blockchain found. Add a block first!");
    }
}

// Validate entire chain integrity with nonce
pub fn validate_chain() {
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

    for i in 0..chain.len() {
        let block = &chain[i];

        // Recalculate hash including nonce
        let recalculated = block.calculate_hash();
        if block.hash != recalculated {
            println!("❌ Invalid hash at block {}", block.index);
            return;
        }

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