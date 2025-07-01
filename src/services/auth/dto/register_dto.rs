use crate::constants::{MAX_USERNAME_LEN, MIN_USERNAME_LEN, validate_password};
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RegisterDto {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = MIN_USERNAME_LEN, max = MAX_USERNAME_LEN))]
    pub username: String,

    #[validate(custom(function = "validate_password"))]
    pub password: String,
}
