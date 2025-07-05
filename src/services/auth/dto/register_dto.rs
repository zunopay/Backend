use crate::constants::{MAX_USERNAME_LEN, MIN_USERNAME_LEN, validate_password};
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RegisterDto {
    #[validate(email)]
    pub email: String,

    #[validate(custom(function = "validate_password"))]
    pub password: String,
}
