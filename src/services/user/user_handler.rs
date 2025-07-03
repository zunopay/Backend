use crate::services::error::Result;
use crate::services::user::dto::create_merchant_profile_dto::{
    CreateMerchantProfileDto, from_multipart_to_create_merchant_profle_dto,
};
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
use axum::extract::{Multipart, Path};
use axum::{Json, extract::State};
use std::sync::Arc;

pub async fn find_me(State(state): State<Arc<AppState>>, ctx: Ctx) -> Result<Json<UserDto>> {
    let user = UserService::find_one(state, ctx.user_id).await?;
    Ok(Json(user.into()))
}

pub async fn find_merchant(
    State(state): State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> Result<Json<MerchantDto>> {
    let merchant = UserService::find_merchant(state, slug).await?;
    Ok(Json(merchant.into()))
}

pub async fn find_all_merchants(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<MerchantDto>>> {
    let merchants = UserService::find_all_merchants(state).await?;
    let merchants = merchants
        .into_iter()
        .map(|val| val.into())
        .collect::<Vec<MerchantDto>>();

    Ok(Json(merchants))
}

pub async fn create_merchant_profile(
    State(state): State<Arc<AppState>>,
    ctx: Ctx,
    mut form: Multipart,
) -> Result<Json<MerchantDto>> {
    let body = from_multipart_to_create_merchant_profle_dto(form).await?;
    let merchant = UserService::create_merchant_profile(state, ctx, body).await?;
    Ok(Json(merchant.into()))
}
