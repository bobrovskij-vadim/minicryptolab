use sha2::{Digest, Sha256};
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct HashEntry {
    text: String,
    hash: String,
}

const HISTORY_FILE: &str = "hash_history.json";

pub fn run(text: String) {
    // 1. Calculate SHA256
    let mut hasher = Sha256::new();
    hasher.update(text.as_bytes());
    let result = hasher.finalize();
    let hash_hex = format!("{:x}", result);

    println!("🔹 Source: {}", text);
    println!("🔐 SHA256 hash: {}", hash_hex);

    // 2. Load the old history (if any)
    let mut history: Vec<HashEntry> = if Path::new(HISTORY_FILE).exists() {
        let data = fs::read_to_string(HISTORY_FILE).unwrap_or_default();
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        Vec::new()
    };

    // 3. Add a new entry
    history.push(HashEntry { text, hash: hash_hex });

    // 4. Save to file
    let json = serde_json::to_string_pretty(&history).unwrap();
    fs::write(HISTORY_FILE, json).expect("Failed to record history");

    println!("💾 History saved in {HISTORY_FILE}");
}