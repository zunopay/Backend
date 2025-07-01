use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginDto {
    pub username_or_email: String,
    pub password: String,
}
