pub(crate) mod dto;
mod payment_handler;

use super::{
    AppState,
    error::{Result, ServiceError},
};
use crate::{
    ctx::Ctx,
    db::entity::{
        payment::{self, Column},
        prelude::Payment,
        user,
    },
    services::{
        append_timestamp, create_transfer_transaction,
        error::EntityId,
        payment::dto::{
            create_payment_dto::CreatePaymentDto,
            payment_dto::{PaymentDto, PaymentInput},
        },
        user::UserService,
    },
};
use ::chrono::{DateTime, Utc, naive};
use axum::extract::State;
use convert_case::{Case, Casing};
use sea_orm::{
    ActiveValue::Set, ColumnTrait, EntityName, EntityTrait, QueryFilter, TransactionTrait,
    prelude::DateTimeWithTimeZone, sqlx::types::chrono,
};
use uuid::Uuid;

pub struct PaymentService;

impl PaymentService {
    const TABLE: &'static str = "Payment";

    pub async fn find_one(state: AppState, id: i32) -> Result<PaymentInput> {
        let payment = Payment::find_by_id(id).one(state.db()).await?;
        payment.ok_or(ServiceError::EntityNotFound {
            entity: Self::TABLE,
            id: EntityId::Int(id),
        })
    }

    //todo: payment id should be a uuid
    pub async fn create(
        state: AppState,
        user_id: i32,
        create_payment_dto: CreatePaymentDto,
    ) -> Result<PaymentInput> {
        let data = payment::ActiveModel {
            title: Set(create_payment_dto.title),
            description: Set(create_payment_dto.description),
            amount: Set(create_payment_dto.amount),
            created_at: Set(Utc::now().naive_utc()),
            category: Set(create_payment_dto.category),
            public_id: Set(Uuid::new_v4()),
            user_id: Set(user_id),
            ..Default::default()
        };

        let payment_link = Payment::insert(data)
            .exec_with_returning(state.db())
            .await?;
        Ok(payment_link)
    }

    pub async fn create_transfer(state: AppState, payment_id: i32) -> Result<String> {
        // Find the payment
        let payment = Payment::find_by_id(payment_id).one(state.db()).await?;
        // validate the transfer (allowlist or other criteria's)
        // create transfer tx and transfer data in db and return to user
        let transfer_transaction = create_transfer_transaction().await?;
        // start watching reference key for transaction status
        Ok(transfer_transaction)
    }

    pub async fn public_create_transfer(state: AppState, payment_id: String) -> Result<String> {
        // Find the payment
        let payment = Payment::find()
            .filter(Column::PublicId.eq(&payment_id))
            .one(state.db())
            .await?;

        if payment.is_none() {
            return Err(ServiceError::EntityNotFound {
                entity: Self::TABLE,
                id: EntityId::Str(payment_id),
            });
        }
        // validate the transfer (allowlist or other criteria's)
        // create transfer tx and transfer data in db and return to user
        let transfer_transaction = create_transfer_transaction().await?;
        // start watching reference key for transaction status
        Ok(transfer_transaction)
    }
}
