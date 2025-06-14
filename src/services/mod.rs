pub mod app;
pub mod auth;
pub mod error;
mod indexer;
pub mod payment;
pub mod s3;
pub mod user;

use crate::{
    config::config,
    db,
    services::{error::Result, s3::S3Service},
};
use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use base64::{Engine, engine::general_purpose};
use sea_orm::{
    Database, DatabaseConnection,
    sqlx::{Postgres, types::time},
};
use std::sync::Arc;

//Cloning is cheap on DatabaseConnection
#[derive(Clone)]
pub struct AppState {
    db: DatabaseConnection,
    s3: Arc<S3Service>,
}

impl AppState {
    pub async fn new() -> crate::error::Result<Self> {
        let db = db::connect_database().await?;

        let s3 = S3Service::new().await;
        let s3 = Arc::new(s3);

        Ok(AppState { db, s3 })
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

pub async fn create_transfer_transaction() -> Result<String> {
    todo!()
}