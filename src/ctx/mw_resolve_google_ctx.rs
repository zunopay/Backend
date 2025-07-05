use std::{str::FromStr, sync::Arc};

use crate::{
    config,
    constants::GOOGLE_OAUTH_BASE_URL,
    ctx::GoogleCtx,
    error::{Error, Result},
    services::{AppState, auth::dto::authorization_dto::GoogleUser},
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
    State(state): State<Arc<AppState>>,
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

    let payload = fetch_google_user_info(token).await?;
    let google_ctx = GoogleCtx {
        email: payload.email,
    };
    request.extensions_mut().insert(google_ctx);

    Ok(next.run(request).await)
}

async fn fetch_google_user_info(access_token: &str) -> Result<GoogleUser> {
    let client = Client::new();
    let response = client
        .get(format!("{}/oauth2/v3/userinfo", GOOGLE_OAUTH_BASE_URL))
        .bearer_auth(access_token)
        .send()
        .await
        .map_err(|_| Error::MiddlewareError("Failed to fetch google user"))?;

    let google_user = response
        .json::<GoogleUser>()
        .await
        .map_err(|_| Error::MiddlewareError("Failed to parse google user"))?;

    Ok(google_user)
}
