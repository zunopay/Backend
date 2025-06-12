use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize)]
pub struct LoginDto {
    pub username_or_email: String,
    pub password: String,
}
