use serde::Serialize;

use crate::{db::entity::merchant, services::get_public_url};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MerchantDto {
    pub display_name: String,
    pub cover: Option<String>,
    pub address: String,
    pub is_verified: bool,
}

impl From<merchant::Model> for MerchantDto {
    fn from(value: merchant::Model) -> Self {
        MerchantDto {
            display_name: value.display_name,
            cover: value.cover.map(|cover_key| get_public_url(&cover_key)),
            address: value.address,
            is_verified: value.is_verified,
        }
    }
}
