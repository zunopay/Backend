use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrivyWallet {
    pub id: Option<String>,
    pub address: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrivyUser {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub wallet: Option<PrivyWallet>,
}
