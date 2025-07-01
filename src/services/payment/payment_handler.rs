use std::sync::Arc;

use crate::{
    ctx::Ctx,
    services::{
        AppState,
        error::Result,
        payment::{
            PaymentService,
            dto::{
                create_payment_dto::CreatePaymentDto, create_transfer_dto::CreateTransferDto,
                payment_dto::PaymentDto,
            },
        },
    },
};
use axum::{
    Json,
    extract::{Multipart, Path, State},
};
use serde_json::json;
use uuid::Uuid;

// pub async fn find_one(
//     State(state): State<Arc<AppState>>,
//     Path(id): Path<i32>,
// ) -> Result<Json<PaymentDto>> {
//     let payment = PaymentService::find_one(state, id).await?;
//     Ok(Json(payment.into()))
// }

pub async fn find_one(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<PaymentDto>> {
    let payment = PaymentService::public_find_one(state, id).await?;
    Ok(Json(payment.into()))
}

pub async fn create(
    State(state): State<Arc<AppState>>,
    ctx: Ctx,
    Json(create_payment_dto): Json<CreatePaymentDto>,
) -> Result<Json<PaymentDto>> {
    let payment_input = PaymentService::create(state, ctx.user_id, create_payment_dto).await?;
    Ok(Json(payment_input.into()))
}

pub async fn create_transfer(
    State(state): State<Arc<AppState>>,
    Json(create_transfer_dto): Json<CreateTransferDto>,
) -> Result<String> {
    let transfer_transaction = PaymentService::create_transfer(state, create_transfer_dto).await?;
    Ok(transfer_transaction)
}
