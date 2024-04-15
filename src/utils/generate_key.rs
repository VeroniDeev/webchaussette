use base64::{engine::general_purpose, Engine};
use rand::{rngs::ThreadRng, RngCore};
use sha1::{Digest, Sha1};

use crate::websocket_types::WEBSOCKET_GUID;

pub fn generate_key() -> String {
    let mut rng: ThreadRng = rand::thread_rng();
    let mut random_bytes: [u8; 16] = [0u8; 16];
    rng.fill_bytes(&mut random_bytes);

    let key: String = general_purpose::STANDARD.encode(random_bytes);
    key
}

pub fn generate_accept(key: String) -> String {
    let mut hasher = Sha1::new();
    let key_concatenate: String = format!("{}{}", key, WEBSOCKET_GUID);

    hasher.update(key_concatenate);
    let hashed = hasher.finalize();
    // let hex_hashed: String = hex::encode(hashed);
    // println!("{}", hex_hashed);

    let accept: String = general_purpose::STANDARD.encode(hashed);
    accept
}
