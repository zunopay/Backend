use crate::{
    ctx::GoogleCtx,
    services::{
        AppState,
        auth::{
            AuthService,
            dto::{
                authorization_dto::AuthorizationDto, login_dto::LoginDto, register_dto::RegisterDto,
            },
        },
        error::Result,
    },
};
use axum::{Json, extract::State};
use validator::Validate;

pub async fn login_with_google(
    State(state): State<AppState>,
    ctx: GoogleCtx,
) -> Result<Json<AuthorizationDto>> {
    let result = AuthService::login_with_google(state, ctx.email).await?;
    Ok(Json(result))
}

pub async fn register(
    State(state): State<AppState>,
    Json(body): Json<RegisterDto>,
) -> Result<Json<AuthorizationDto>> {
    body.validate()?;

    let result = AuthService::register(state, body).await?;
    Ok(Json(result))
}

pub async fn login(
    State(state): State<AppState>,
    Json(body): Json<LoginDto>,
) -> Result<Json<AuthorizationDto>> {
    let result = AuthService::login(state, body).await?;
    Ok(Json(result))
}
