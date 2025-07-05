use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct AuthorizationDto {
    pub auth_token: String,
    // pub refresh_token: String (TODO)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BasicUserPayload {
    pub user_id: i32,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Claims {
    #[serde(flatten)]
    pub user: BasicUserPayload,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Debug, Deserialize)]
pub struct GoogleUser {
    pub sub: String,
    pub email: String,
    pub name: String,
    pub picture: String,
}
