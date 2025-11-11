# MiniCryptoLab

**MiniCryptoLab** is an educational Rust project demonstrating the core principles of blockchain, cryptography, and Proof of Work. The project is designed to help learn Rust while building a working mini-blockchain.

---

## 📦 Features

1. **CLI**: Full control of the project via command line.
2. **Hash**: SHA256 hashing of text with persistent history.
3. **Blockchain**: Creation of blocks with `prev_hash` and `nonce`.
4. **Keys**: ECDSA key generation and block signing.
5. **Validate Chain**: Check the integrity of the blockchain.
6. **Validate Signatures**: Verify signatures of all blocks.
7. **Proof of Work**: Mine blocks with configurable difficulty.

---

## ⚡ Installation

1. Clone the repository:

```bash
git clone https://github.com/yourusername/minicryptolab.git
cd minicryptolab
```

2. Install Rust if not already installed:  
[https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

3. Build the project:

```bash
cargo build
```

---

## 🛠 CLI Commands

```bash
cargo run -- hash "Hello world"           # Calculate SHA256 hash
cargo run -- history                      # Show hash history
cargo run -- generate-keys                # Generate ECDSA keys
cargo run -- add-block "Some data" 4      # Add a block with mining (difficulty = 4)
cargo run -- show-chain                   # Show all blocks in the blockchain
cargo run -- validate-chain               # Validate blockchain integrity
cargo run -- validate-signatures          # Verify all block signatures
```

---

## 📜 Example Output

```bash
cargo run -- add-block "Hello POW" 4
⛏️ Start mining block with difficulty 4...
⛏️ Mining... nonce = 0, hash = 7a2f3b9c
⛏️ Mining... nonce = 10000, hash = 00a1b2c3
✅ Block mined! nonce = 15678, hash = 0000f12ab34c56789
🔏 Block signed successfully!
✅ Block added to blockchain!
```

---

## 🧩 Technologies & Libraries

- **Rust** — programming language
- **Clap** — CLI parser
- **SHA256 (sha2 crate)** — hashing
- **P256 (p256 crate)** — ECDSA signatures
- **Serde** — JSON serialization/deserialization
- **Chrono** — timestamps

---

## 🎯 Project Goals

- Learn Rust and its ecosystem
- Understand blockchain and Proof of Work
- Practice cryptography (SHA256 & ECDSA)

---

## 📂 Project Structure

```
minicryptolab/
├── src/
│   ├── main.rs       # CLI and entry point
│   ├── hash.rs       # SHA256 hashing module
│   ├── blockchain.rs # Blockchain and Proof of Work
│   ├── keys.rs       # ECDSA key generation & loading
├── Cargo.toml
├── hash_history.json  # Persistent hash history
├── blockchain.json    # Stored blockchain data
├── README.md
```

---

## ✅ Notes

- Difficulty can be adjusted in the `add-block` command to make mining harder or easier.
- Make sure to run `generate-keys` before adding blocks to enable block signing.
- All data is stored locally in JSON files (`hash_history.json` and `blockchain.json`).

