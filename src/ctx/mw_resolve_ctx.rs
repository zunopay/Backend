use crate::{
    config,
    constants::AUTH_PREFIX,
    ctx::{Ctx, CtxResult},
    error::{Error, Result},
    services::{AppState, auth::dto::authorization_dto::Claims, user::UserService},
};
use axum::{
    extract::{Request, State},
    http::{HeaderMap, header::AUTHORIZATION},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{DecodingKey, TokenData, Validation, decode};
use sea_orm::EntityTrait;
use std::{string::ToString, sync::Arc};
use tower_cookies::{Cookie, Cookies};

#[axum::debug_middleware]
pub async fn mw_resolve_ctx(
    State(state): State<Arc<AppState>>,
    mut request: Request,
    next: Next,
) -> Result<Response> {
    let headers = request.headers().clone();
    let ctx_result = _mw_resolve_ctx(state, headers).await;
    request.extensions_mut().insert(ctx_result);

    Ok(next.run(request).await)
}

async fn _mw_resolve_ctx(state: Arc<AppState>, headers: HeaderMap) -> CtxResult {
    let token_str = headers
        .get(AUTHORIZATION)
        .ok_or(Error::MissingAuthToken)?
        .to_str()
        .map_err(|_| Error::ServiceError("Failed to parse token".to_string()))?
        .strip_prefix(AUTH_PREFIX)
        .ok_or(Error::ServiceError("Invalid AuthToken".to_string()))?;

    let key = DecodingKey::from_secret(config::config().ACCESS_SECRET_KEY.as_bytes());

    //TODO: Check if HS256 is the default validation
    let auth_token: TokenData<Claims> = decode(
        &token_str,
        &key,
        &Validation::new(jsonwebtoken::Algorithm::HS256),
    )?;

    let user = UserService::find_one(state, auth_token.claims.user.user_id).await?;

    //todo: verify user role
    let ctx = Ctx { user_id: user.id };

    Ok(ctx)
}
