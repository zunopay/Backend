use crate::services::error::Result;
use crate::services::user::dto::create_merchant_profile_dto::CreateMerchantProfileDto;
use crate::{
    ctx::Ctx,
    services::{
        AppState,
        user::{
            UserService,
            dto::{merchant_dto::MerchantDto, user_dto::UserDto},
        },
    },
};
use axum::{Json, extract::State};
use std::sync::Arc;

pub async fn find_me(State(state): State<Arc<AppState>>, ctx: Ctx) -> Result<UserDto> {
    let user = UserService::find_one(state, ctx.user_id).await?;
    Ok(user.into())
}

pub async fn find_merchant(
    State(state): State<Arc<AppState>>,
    slug: String,
) -> Result<MerchantDto> {
    let merchant = UserService::find_merchant(state, slug).await?;
    Ok(merchant.into())
}

pub async fn find_all_merchants(State(state): State<Arc<AppState>>) -> Result<Vec<MerchantDto>> {
    let merchants = UserService::find_all_merchants(state).await?;
    let merchants = merchants
        .into_iter()
        .map(|val| val.into())
        .collect::<Vec<MerchantDto>>();

    Ok(merchants)
}

pub async fn create_merchant_profile(
    State(state): State<Arc<AppState>>,
    ctx: Ctx,
    Json(body): Json<CreateMerchantProfileDto>,
) -> Result<MerchantDto> {
    let merchant = UserService::create_merchant_profile(state, ctx, body).await?;
    Ok(merchant.into())
}
