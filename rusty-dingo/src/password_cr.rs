/// Experimental Password Cracker made purely for experimental use
/// of Crypto and other related modules
/// Not for production usage
/// 
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use rayon::prelude::*;
use cracker_db::{ PasswordHash, Db };
use sqlx::Error;


async fn hash_password(password: &str) -> Result<PasswordHash, Error> {
    let mut hasher = Sha256::new();
    hasher.hash_password().input_str(password);
    let hash: &str = hasher.result_str();
    save_password_hash(hash, password).await?;
    Ok(hash)
}

/// Save password hash - can be used for login screens
async fn save_password_hash(hash: &PasswordHash, password: &str) -> Result<(), Error> {
    let hash: &PasswordHash = hash.as_string()
    let db: Db = cracker_db::Db::init();
    match db.insert_hash(id, &hash, status, password)
        .await? {
            Ok(hash) => println!("{} saved successfully", hash),
            Err(e) => eprintln!("Error: {}", e),
        }
}