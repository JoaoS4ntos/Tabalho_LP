use argon2::{self, Config};
use rand::Rng;

pub fn hash_password(password: &str) -> Result<String, Box<dyn std::error::Error>> {
    let salt: [u8; 16] = rand::thread_rng().gen();
    let config = Config::default();
    let hash = argon2::hash_encoded(password.as_bytes(), &salt, &config)?;
    Ok(hash)
}

pub fn verify_password(hash: &str, password: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let matches = argon2::verify_encoded(hash, password.as_bytes())?;
    Ok(matches)
}