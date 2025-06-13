use crate::{
    ctx::Ctx,
    services::{
        AppState,
        error::Result,
        payment::{
            PaymentService,
            dto::{create_payment_dto::CreatePaymentDto, payment_dto::PaymentDto},
        },
    },
};
use axum::{
    Json,
    extract::{Multipart, Path, State},
};
use serde_json::json;

pub async fn find_one(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<PaymentDto>> {
    let payment = PaymentService::find_one(state, id).await?;
    Ok(Json(payment.into()))
}

pub async fn create(
    State(state): State<AppState>,
    ctx: Ctx,
    Json(create_payment_dto): Json<CreatePaymentDto>,
) -> Result<Json<PaymentDto>> {
    let payment_input = PaymentService::create(state, ctx.user_id, create_payment_dto).await?;
    Ok(Json(payment_input.into()))
}
