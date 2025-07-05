use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginDto {
    pub email: String,
    pub password: String,
}
