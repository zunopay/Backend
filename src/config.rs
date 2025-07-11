use std::{env, sync::OnceLock};

use crate::error::{Error, Result};

#[allow(non_snake_case)]
pub struct Config {
    pub PORT: String,
    pub DB_URL: String,
    pub ACCESS_SECRET_KEY: String,
    pub AWS_BUCKET_NAME: String,
    pub AWS_ACCESS_KEY_ID: String,
    pub AWS_SECRET_ACCESS_KEY: String,
    pub AWS_BUCKET_REGION: String,
    pub RPC_URL: String,
    pub FEE_FAUCET_SECRET: String,
    pub FEE_FAUCET_PRIVATE_KEY: String,
    pub GOOGLE_OAUTH_CLIENT_ID: String,
    pub GOOGLE_OAUTH_CLIENT_SECRET: String,
    pub PRIVY_APP_ID: String,
    pub PRIVY_APP_SECRET: String,
}

pub fn config() -> &'static Config {
    static CONFIG: OnceLock<Config> = OnceLock::new();

    let config = CONFIG.get_or_init(|| {
        Config::load_env()
            .unwrap_or_else(|e| panic!("FATAL - ERROR WHILE LOADING CONFIG ENV - CAUSE: {}", e))
    });

    config
}

impl Config {
    pub fn load_env() -> Result<Config> {
        let config = Config {
            PORT: get_var("PORT")?,
            DB_URL: get_var("SERVICE_DB_URL")?,
            ACCESS_SECRET_KEY: get_var("SERVICE_ACCESS_SECRET_KEY")?,
            AWS_BUCKET_NAME: get_var("SERVICE_AWS_BUCKET_NAME")?,
            AWS_ACCESS_KEY_ID: get_var("SERVICE_AWS_ACCESS_KEY_ID")?,
            AWS_SECRET_ACCESS_KEY: get_var("SERVICE_AWS_SECRET_ACCESS_KEY")?,
            AWS_BUCKET_REGION: get_var("SERVICE_AWS_BUCKET_REGION")?,
            RPC_URL: get_var("SERVICE_RPC_URL")?,
            FEE_FAUCET_SECRET: get_var("SERVICE_FEE_FAUCET_SECRET")?,
            FEE_FAUCET_PRIVATE_KEY: get_var("SERVICE_FEE_FAUCET_PRIVATE_KEY")?,
            GOOGLE_OAUTH_CLIENT_ID: get_var("SERVICE_GOOGLE_CLIENT_ID")?,
            GOOGLE_OAUTH_CLIENT_SECRET: get_var("SERVICE_GOOGLE_CLIENT_SECRET")?,
            PRIVY_APP_ID: get_var("SERVICE_PRIVY_APP_ID")?,
            PRIVY_APP_SECRET: get_var("SERVICE_PRIVY_APP_SECRET")?,
        };

        Ok(config)
    }
}

fn get_var(key: &'static str) -> Result<String> {
    env::var(key).map_err(|_| Error::EnvMissing(key))
}
