// use sha2::{Digest, Sha256};
// use serde::{Serialize, Deserialize};
// use std::fs;
// use std::path::Path;
// use chrono::Utc;
// use p256::ecdsa::{signature::Signer, signature::Verifier, Signature, SigningKey, VerifyingKey};
// use crate::keys;

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct Block {
//     index: u64,
//     timestamp: String,
//     data: String,
//     prev_hash: String,
//     hash: String,
//     signature: Option<String>,
//     public_key: Option<String>,
// }

// const CHAIN_FILE: &str = "blockchain.json";

// impl Block {
//     pub fn new(index: u64, data: String, prev_hash: String) -> Self {
//         let timestamp = Utc::now().to_rfc3339();

//         // Combine block fields for hashing
//         let record = format!("{}{}{}{}", index, timestamp, data, prev_hash);
//         let hash = Self::calculate_hash_raw(&record);

//         Block {
//             index,
//             timestamp,
//             data,
//             prev_hash,
//             hash,
//             signature: None,
//             public_key: None,
//         }
//     }

//     fn calculate_hash_raw(record: &str) -> String {
//         let mut hasher = Sha256::new();
//         hasher.update(record.as_bytes());
//         format!("{:x}", hasher.finalize())
//     }

//     fn calculate_hash(&self) -> String {
//         let record = format!("{}{}{}{}", self.index, self.timestamp, self.data, self.prev_hash);
//         Self::calculate_hash_raw(&record)
//     }

//     pub fn sign(&mut self, key: &SigningKey, pubkey: &VerifyingKey) {
//         // Sign block hash with private key
//         let signature: Signature = key.sign(self.hash.as_bytes());

//         self.signature = Some(hex::encode(signature.to_der()));
//         self.public_key = Some(hex::encode(pubkey.to_encoded_point(false).as_bytes()));
//     }

//     pub fn verify_signature(&self) -> bool {
//         if let (Some(sig_hex), Some(pub_hex)) = (&self.signature, &self.public_key) {
//             let sig_bytes = hex::decode(sig_hex).ok()?;
//             let pub_bytes = hex::decode(pub_hex).ok()?;
//             let signature = Signature::from_der(&sig_bytes).ok()?;
//             let point = p256::EncodedPoint::from_bytes(&pub_bytes).ok()?;
//             let pubkey = VerifyingKey::from_encoded_point(&point).ok()?;

//             pubkey.verify(self.hash.as_bytes(), &signature).is_ok()
//         } else {
//             false
//         }
//     }
// }

// pub fn add_block(data: String) {
//     let mut chain: Vec<Block> = if Path::new(CHAIN_FILE).exists() {
//         let content = fs::read_to_string(CHAIN_FILE).unwrap_or_default();
//         serde_json::from_str(&content).unwrap_or_default()
//     } else {
//         Vec::new()
//     };

//     let prev_hash = if let Some(last_block) = chain.last() {
//         last_block.hash.clone()
//     } else {
//         String::from("GENESIS")
//     };

//     let mut block = Block::new(chain.len() as u64, data, prev_hash);

//     // Load keys and sign the block
//     if let (Some(priv_key), Some(pub_key)) = (keys::load_private_key(), keys::load_public_key()) {
//         block.sign(&priv_key, &pub_key);
//         println!("🔏 Block signed successfully!");
//     } else {
//         println!("⚠️ No keys found. Run 'generate-keys' first.");
//     }

//     chain.push(block);
//     let json = serde_json::to_string_pretty(&chain).unwrap();
//     fs::write(CHAIN_FILE, json).expect("Failed to write blockchain");
//     println!("✅ Block added to blockchain!");
// }

// pub fn validate_signatures() {
//     let content = fs::read_to_string(CHAIN_FILE);
//     if content.is_err() {
//         println!("❌ Blockchain not found!");
//         return;
//     }

//     let chain: Vec<Block> = serde_json::from_str(&content.unwrap()).unwrap_or_default();
//     if chain.is_empty() {
//         println!("❌ Blockchain is empty!");
//         return;
//     }

//     for block in &chain {
//         if block.signature.is_none() {
//             println!("⚠️ Block {} is unsigned.", block.index);
//             continue;
//         }

//         if block.verify_signature() {
//             println!("✅ Block {} signature is valid.", block.index);
//         } else {
//             println!("❌ Block {} signature is INVALID!", block.index);
//         }
//     }
// }

// pub fn show_chain() {
//     if let Ok(data) = fs::read_to_string(CHAIN_FILE) {
//         println!("📦 Blockchain:\n{}", data);
//     } else {
//         println!("❌ No blockchain found. Add a block first!");
//     }
// }

// pub fn validate_chain() {
//     let content = fs::read_to_string(CHAIN_FILE);
//     if content.is_err() {
//         println!("❌ Blockchain file not found!");
//         return;
//     }

//     let chain: Vec<Block> = serde_json::from_str(&content.unwrap()).unwrap_or_default();

//     if chain.is_empty() {
//         println!("❌ Blockchain is empty!");
//         return;
//     }

//     for i in 0..chain.len() {
//         let block = &chain[i];

//         // Recalculate hash and compare
//         let recalculated = block.calculate_hash();
//         if block.hash != recalculated {
//             println!("❌ Invalid hash at block {}", block.index);
//             return;
//         }

//         // Compare previous hash (skip genesis)
//         if i > 0 {
//             let prev = &chain[i - 1];
//             if block.prev_hash != prev.hash {
//                 println!("❌ Broken chain at block {}", block.index);
//                 return;
//             }
//         }
//     }

//     println!("✅ Blockchain integrity verified. All blocks are valid!");
// }

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
}

const CHAIN_FILE: &str = "blockchain.json";

impl Block {
    pub fn new(index: u64, data: String, prev_hash: String) -> Self {
        let timestamp = Utc::now().to_rfc3339();

        // Combine fields to form a hash base string
        let record = format!("{}{}{}{}", index, timestamp, data, prev_hash);
        let hash = Self::calculate_hash_raw(&record);

        Block {
            index,
            timestamp,
            data,
            prev_hash,
            hash,
            signature: None,
            public_key: None,
        }
    }

    fn calculate_hash_raw(record: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(record.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    fn calculate_hash(&self) -> String {
        let record = format!("{}{}{}{}", self.index, self.timestamp, self.data, self.prev_hash);
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
            // Decode hex safely
            let sig_bytes = match hex::decode(sig_hex) {
                Ok(v) => v,
                Err(_) => return false,
            };
            let pub_bytes = match hex::decode(pub_hex) {
                Ok(v) => v,
                Err(_) => return false,
            };

            // Recreate signature and verifying key
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

pub fn add_block(data: String) {
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

    let mut block = Block::new(chain.len() as u64, data, prev_hash);

    // Load keys and sign
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

// Validate entire chain integrity
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

        // Recalculate hash and compare
        let recalculated = block.calculate_hash();
        if block.hash != recalculated {
            println!("❌ Invalid hash at block {}", block.index);
            return;
        }

        // Compare previous hash (skip genesis)
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