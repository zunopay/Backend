pub(crate) mod dto;
mod payment_handler;

use super::{
    AppState,
    error::{Result, ServiceError},
};
use crate::{
    ctx::Ctx,
    db::entity::{Payment, PaymentEntity, payment, user},
    services::{
        append_timestamp,
        payment::dto::{
            create_payment_dto::CreatePaymentDto,
            payment_dto::{PaymentDto, PaymentInput},
        },
        user::UserService,
    },
};
use ::chrono::{DateTime, Utc};
use axum::extract::State;
use convert_case::{Case, Casing};
use sea_orm::{
    ActiveValue::Set, EntityName, EntityTrait, TransactionTrait, prelude::DateTimeWithTimeZone,
    sqlx::types::chrono,
};

pub struct PaymentService;

impl PaymentService {
    const TABLE: &'static str = "Payment";

    pub async fn find_one(state: AppState, id: i32) -> Result<PaymentInput> {
        let payment = PaymentEntity::find_by_id(id).one(state.db()).await?;
        payment.ok_or(ServiceError::EntityNotFound {
            entity: Self::TABLE,
            id,
        })
    }

    pub async fn create(
        state: AppState,
        user_id: i32,
        create_payment_dto: CreatePaymentDto,
    ) -> Result<PaymentInput> {
        let data = payment::ActiveModel {
            title: Set(create_payment_dto.title),
            description: Set(create_payment_dto.description),
            amount: Set(create_payment_dto.amount),
            created_at: Set(Utc::now()),
            category: Set(create_payment_dto.category),
            user_id: Set(user_id),
            ..Default::default()
        };

        let payment_link = PaymentEntity::insert(data)
            .exec_with_returning(state.db())
            .await?;
        Ok(payment_link)
    }
}
