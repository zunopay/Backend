use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct AuthorizationDto {
    pub auth_token: String,
    // pub refresh_token: String (TODO)
}

#[derive(Serialize, Deserialize)]
pub struct BasicUserPayload {
    pub user_id: i32,
    pub username: String,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    #[serde(flatten)]
    pub user: BasicUserPayload,
    pub iat: usize,
    pub exp: usize,
}
