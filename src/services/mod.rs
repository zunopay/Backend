pub mod app;
pub mod auth;
pub mod error;
mod indexer;
pub mod payment;
pub mod s3;
pub mod user;
pub mod web3;

use std::sync::Arc;

use crate::{
    config::config,
    db,
    services::{
        error::{Result, ServiceError},
        indexer::Indexer,
        s3::S3Service,
        web3::Web3Service,
    },
};
use aes_gcm::{Aes256Gcm, AesGcm, KeyInit, aead::Aead};
use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use base64::{Engine, engine::general_purpose};
use rand::Rng;
use sea_orm::{
    Database, DatabaseConnection,
    sqlx::{Postgres, types::time},
};
use sha2::{Digest, Sha256, digest::generic_array::GenericArray};
use solana_keypair::Keypair;
use solana_signer::Signer;

//Cloning is cheap on DatabaseConnection
#[derive(Clone)]
pub struct AppState {
    db: DatabaseConnection,
    s3: Arc<S3Service>,
    web3: Arc<Web3Service>,
}

impl AppState {
    pub async fn new() -> crate::error::Result<Self> {
        let db = db::connect_database().await?;

        let s3 = S3Service::new().await;
        let s3 = Arc::new(s3);

        let web3 = Arc::new(Web3Service::new()?);

        Ok(AppState { db, s3, web3 })
    }

    // Access db on in services
    pub(in crate::services) fn db(self: &Self) -> &DatabaseConnection {
        &self.db
    }
}

pub fn hash_password(password: String) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let hash = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(hash.to_string())
}

pub fn verify_password(password: String, hashed_password: String) -> Result<()> {
    let argon2 = Argon2::default();
    let parsed_password_hash = PasswordHash::new(&hashed_password)?;
    argon2.verify_password(password.as_bytes(), &parsed_password_hash)?;

    Ok(())
}

pub fn append_timestamp(value: &String) -> String {
    let timestamp = chrono::Utc::now().timestamp().to_string();

    let mut appended_value = value.clone();
    appended_value.push_str(&format!("-{}", timestamp));

    appended_value
}

pub fn get_public_url(key: &String) -> String {
    let url = format!(
        "https://{}.s3.amazonaws.com/{}",
        config().AWS_BUCKET_NAME,
        key
    );

    url
}

pub fn create_wallet() -> Result<(String, String, String)> {
    let wallet = Keypair::new();
    // generate a random secret for encoding keypair
    let mut secret = [0u8; 32];
    rand::rng().fill(&mut secret);

    // Create cipher from secret
    let sha256_secret = Sha256::digest(secret);
    let cipher = Aes256Gcm::new_from_slice(&sha256_secret).map_err(|_| {
        ServiceError::Web3Error(error::Web3ErrorType::Custom(
            "Failed to create AES secret".to_string(),
        ))
    })?;

    // Create a random nonce
    let mut nonce = [0u8; 12];
    rand::rng().fill(&mut nonce);

    // Encrypt wallet with nonce using cipher
    let ciphertext = cipher
        .encrypt(GenericArray::from_slice(&nonce), wallet.to_bytes().as_ref())
        .map_err(|_| {
            ServiceError::Web3Error(error::Web3ErrorType::Custom(
                "Failed to encrypt wallet".to_string(),
            ))
        })?;

    //Base64 encode the result
    let mut result = vec![];
    result.extend_from_slice(&nonce);
    result.extend_from_slice(&ciphertext);

    let encrypted_wallet = general_purpose::STANDARD.encode(result);
    let encrypted_secret = general_purpose::STANDARD.encode(secret);

    Ok((
        encrypted_wallet,
        encrypted_secret,
        wallet.pubkey().to_string(),
    ))
}

fn decode_keypair(private_key: &String, secret: &String) -> Result<Keypair> {
    let private_key_bytes = general_purpose::STANDARD.decode(private_key).map_err(|_| {
        ServiceError::Web3Error(error::Web3ErrorType::Custom(
            "Failed to decode private key".to_string(),
        ))
    })?;
    let secret_bytes = general_purpose::STANDARD.decode(secret).map_err(|_| {
        ServiceError::Web3Error(error::Web3ErrorType::Custom(
            "Failed to decode secret".to_string(),
        ))
    })?;

    let (nonce, wallet) = private_key_bytes
        .split_at_checked(12)
        .ok_or(ServiceError::Web3Error(error::Web3ErrorType::Custom(
            "Invalid private key".to_string(),
        )))?;

    let sha256_secret = Sha256::digest(secret_bytes);
    let cipher = Aes256Gcm::new_from_slice(&sha256_secret).map_err(|_| {
        ServiceError::Web3Error(error::Web3ErrorType::Custom(
            "Failed to create AES secret".to_string(),
        ))
    })?;

    let decrypted_wallet = cipher
        .decrypt(GenericArray::from_slice(&nonce), wallet)
        .map_err(|_| {
            ServiceError::Web3Error(error::Web3ErrorType::Custom(
                "Failed to decrypt wallet".to_string(),
            ))
        })?;

    let wallet = Keypair::from_bytes(&decrypted_wallet).map_err(|_| {
        ServiceError::Web3Error(error::Web3ErrorType::Custom(
            "Failed to derive keypair from wallet".to_string(),
        ))
    })?;

    Ok(wallet)
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use crate::services::{create_wallet, decode_keypair};
    use anyhow::Result;
    use solana_signer::Signer;
    use spl_token::solana_program::pubkey::Pubkey;

    #[test]
    fn test_create_wallet() -> Result<()> {
        let (private_key, secret, public_key) = create_wallet()?;
        println!("Public Key: {}", public_key);
        println!("Private key: {}", private_key);
        println!("Secret: {}", secret);

        let keypair = decode_keypair(&private_key, &secret)?;
        assert!(keypair.pubkey().eq(&Pubkey::from_str(&public_key)?));

        Ok(())
    }
}
