use std::str::FromStr;

use crate::{
    config,
    constants::GOOGLE_OAUTH_BASE_URL,
    ctx::GoogleCtx,
    error::{Error, Result},
    services::{
        AppState,
        auth::dto::authorization_dto::{GoogleClaims, GoogleJwks},
    },
};
use axum::{
    Json,
    extract::{Request, State},
    http::header::AUTHORIZATION,
    middleware::Next,
    response::Response,
};
use chrono::{NaiveDate, Utc};
use jsonwebtoken::{DecodingKey, Validation, decode, decode_header};
use reqwest::{Client, StatusCode, Url};
use serde::Deserialize;

pub async fn mw_resolve_google_ctx(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response> {
    let mut auth_token = request
        .headers()
        .get(AUTHORIZATION)
        .ok_or(Error::MissingAuthToken)?
        .to_str()
        .map_err(|_| Error::ServiceError("Failed to parse token".to_string()))?
        .splitn(2, ' ');

    let token_type = auth_token.next().ok_or(Error::MissingAuthToken)?;
    let token = auth_token.next().ok_or(Error::MissingAuthToken)?;

    // Attach "Google" prefix on frontend
    if token_type != "Google" {
        return Err(Error::MissingAuthToken);
    }

    let payload = verify_google_token(token).await?;
    let google_ctx = GoogleCtx {
        email: payload.email,
    };
    request.extensions_mut().insert(google_ctx);

    Ok(next.run(request).await)
}

async fn verify_google_token(token: &str) -> Result<GoogleClaims> {
    // Decode google jwt with jwks

    let jwks = fetch_google_jwks().await?;

    let jwt_header = decode_header(token)?;
    let kid = jwt_header
        .kid
        .ok_or(Error::MiddlewareError("Missing kid in google jwt"))?;

    let jwk = jwks
        .keys
        .iter()
        .find(|jwk| jwk.kid == kid)
        .ok_or(Error::MiddlewareError(
            "No matching kid found in google jwt",
        ))?;

    let decoding_key = DecodingKey::from_rsa_components(&jwk.n, &jwk.e)?;
    let mut validation = Validation::new(jsonwebtoken::Algorithm::RS256);
    validation.set_issuer(&["https://accounts.google.com"]);
    validation.set_audience(&[&config::config().GOOGLE_OAUTH_CLIENT_ID]);

    let jwt_data = decode::<GoogleClaims>(token, &decoding_key, &validation)?;
    let claims = jwt_data.claims;

    // Validate email and exp
    if !claims.email_verified {
        return Err(Error::MiddlewareError("Google email is not verified"));
    }

    let now = usize::try_from(Utc::now().timestamp())
        .map_err(|_| Error::MiddlewareError("Failed to parse usize from i64 timestamp"))?;

    if claims.exp <= now {
        return Err(Error::MiddlewareError("Google jwt is expired"));
    }

    Ok(claims)
}

// JWKS: Json web key set -> to verify google auth token.
async fn fetch_google_jwks() -> Result<GoogleJwks> {
    let client = Client::new();
    let url = format!("{}/oauth2/v3/certs", GOOGLE_OAUTH_BASE_URL);
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|_| Error::MiddlewareError("Failed to fetch google oauth certs"))?;

    let jwks = response
        .json::<GoogleJwks>()
        .await
        .map_err(|_| Error::MiddlewareError("Failed to parse google jwks"))?;
    Ok(jwks)
}
