use std::str::FromStr;

use crate::{
    ctx::GoogleCtx,
    error::{Error, Result},
    services::AppState,
};
use axum::{
    Json,
    extract::{Request, State},
    http::header::AUTHORIZATION,
    middleware::Next,
    response::Response,
};
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

#[derive(Deserialize)]
pub struct GoogleUserPayload {
    email: String,
    email_verified: String,
    sub: String,
    name: Option<String>,
}

async fn verify_google_token(token: &str) -> Result<GoogleUserPayload> {
    let client = Client::new();
    let url = format!("https://oauth2.googleapis.com/tokeninfo?id_token={}", token);
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|_| Error::MiddlewareError("Failed to fetch google auth token details"))?;

    if response.status() != StatusCode::OK {
        return Err(Error::MiddlewareError("Invalid google auth token"));
    };

    let payload = response
        .json::<GoogleUserPayload>()
        .await
        .map_err(|_| Error::MiddlewareError("Invalid google auth token"))?;

    // todo: check email verified from payload

    Ok(payload)
}
