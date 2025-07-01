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
    pub username: String,
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
#[serde(rename_all = "camelCase")]
pub struct GoogleClaims {
    pub iss: String,
    pub sub: String,
    pub aud: String,
    pub exp: usize,
    pub email: String,
    pub email_verified: bool,
    name: Option<String>,
    picture: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GoogleJwk {
    pub kid: String,
    pub n: String,
    pub e: String,
    pub kty: String,
    pub alg: String,
    use_: String,
}

#[derive(Debug, Deserialize)]
pub struct GoogleJwks {
    pub keys: Vec<GoogleJwk>,
}
