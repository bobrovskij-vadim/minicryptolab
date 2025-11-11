use p256::ecdsa::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use std::fs;
use std::path::Path;

const PRIVATE_KEY_FILE: &str = "private_key.der";
const PUBLIC_KEY_FILE: &str = "public_key.der";

// Generate new ECDSA keypair and save to files
pub fn generate_keys() {
    let signing_key = SigningKey::random(&mut OsRng);
    let verifying_key = VerifyingKey::from(&signing_key);

    fs::write(PRIVATE_KEY_FILE, signing_key.to_bytes()).expect("Failed to save private key");
    fs::write(PUBLIC_KEY_FILE, verifying_key.to_encoded_point(false).as_bytes())
        .expect("Failed to save public key");

    println!("🔑 Keys generated successfully!");
    println!("📄 Private key: {}", PRIVATE_KEY_FILE);
    println!("📄 Public key:  {}", PUBLIC_KEY_FILE);
}

// Load private key from file
pub fn load_private_key() -> Option<SigningKey> {
    if !Path::new(PRIVATE_KEY_FILE).exists() {
        return None;
    }

    let bytes = fs::read(PRIVATE_KEY_FILE).ok()?;
    let array: [u8; 32] = bytes.try_into().ok()?;

    // ✅ Convert &[u8; 32] into GenericArray reference
    Some(SigningKey::from_bytes((&array).into()).ok()?)
}

// Load public key from file
pub fn load_public_key() -> Option<VerifyingKey> {
    if !Path::new(PUBLIC_KEY_FILE).exists() {
        return None;
    }

    let bytes = fs::read(PUBLIC_KEY_FILE).ok()?;
    let point = p256::EncodedPoint::from_bytes(&bytes).ok()?;
    VerifyingKey::from_encoded_point(&point).ok()
}