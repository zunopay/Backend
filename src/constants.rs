use std::ops::Deref;

use once_cell::sync::Lazy;
use regex::Regex;
use validator::ValidationError;

pub const MAX_USERNAME_LEN: u64 = 10;
pub const MIN_USERNAME_LEN: u64 = 4;

pub const AUTHORIZATION: &str = "Authorization";
pub const AUTH_PREFIX: &str = "Bearer ";

pub fn validate_password(password: &String) -> Result<(), ValidationError> {
    if password.len() < 8 {
        return Err(ValidationError::new("too_short"));
    }
    if !password.chars().any(|c| c.is_ascii_lowercase()) {
        return Err(ValidationError::new("no_lowercase"));
    }
    if !password.chars().any(|c| c.is_ascii_uppercase()) {
        return Err(ValidationError::new("no_uppercase"));
    }
    if !password.chars().any(|c| c.is_ascii_digit()) {
        return Err(ValidationError::new("no_digit"));
    }

    Ok(())
}

pub const USDC_MINT: &'static str = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
pub const BASE_USDC: u64 = 100_00_00;
pub const TREASURY_PUBKEY: &'static str = "7SMfVRrJw75vPzHCQ3ckUCT9igMRre8VHmodTbaVv4R";

pub const GOOGLE_OAUTH_BASE_URL: &'static str = "https://oauth2.googleapis.com";
pub const PRIVY_BASE_URL: &'static str = "https://auth.privy.io/api";
