use base64::{engine::general_purpose, Engine};
use rand::{rngs::ThreadRng, RngCore};

pub fn generate_key() -> String {
    let mut rng: ThreadRng = rand::thread_rng();
    let mut random_bytes: [u8; 16] = [0u8; 16];
    rng.fill_bytes(&mut random_bytes);

    let key: String = general_purpose::STANDARD.encode(random_bytes);
    key
}
