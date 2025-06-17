pub(crate) mod dto;
mod payment_handler;

use super::{
    AppState,
    error::{Result, ServiceError},
};
use crate::{
    constants::USDC_MINT,
    ctx::Ctx,
    db::entity::{
        payment::{self, Column},
        prelude::Payment,
        sea_orm_active_enums::TransferStatus,
        transfer::{self, ActiveModel as TransferModel},
        user,
    },
    services::{
        append_timestamp,
        error::EntityId,
        payment::dto::{
            create_payment_dto::CreatePaymentDto,
            create_transfer_dto::CreateTransferDto,
            payment_dto::{PaymentDto, PaymentInput},
        },
        user::UserService,
    },
};
use ::chrono::{DateTime, Utc, naive};
use axum::extract::State;
use base64::Engine;
use convert_case::{Case, Casing};
use sea_orm::{
    ActiveValue::Set, ColumnTrait, EntityName, EntityTrait, QueryFilter, TransactionTrait,
    prelude::DateTimeWithTimeZone, sqlx::types::chrono,
};
use serde::Serialize;
use solana_client::rpc_client::SerializableTransaction;
use solana_keypair::{Keypair, signable::Signable};
use solana_signer::Signer;
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

    pub async fn create_transfer(
        state: AppState,
        create_transfer_dto: CreateTransferDto,
    ) -> Result<String> {
        let payment_id = create_transfer_dto.payment_id.to_string();
        let sender_address = create_transfer_dto.sender_address;

        // Find the payment
        let payment = Payment::find()
            .filter(Column::PublicId.eq(&payment_id))
            .find_also_related(user::Entity)
            .one(state.db())
            .await?;

        let (payment, user) = payment.ok_or(ServiceError::EntityNotFound {
            entity: Self::TABLE,
            id: EntityId::Str(payment_id),
        })?;

        let user = user.ok_or(ServiceError::UserNotFound)?;
        let receiver_address = user.wallet_address.ok_or(ServiceError::EntityNotFound {
            entity: "UserWallet",
            id: EntityId::Int(user.id),
        })?;

        let reference = Keypair::new();
        let reference = reference.pubkey();
        let reference_key = reference.to_string();

        // validate the transfer (allowlist or other criteria's)
        // create transfer tx
        let transfer_transaction = state
            .web3
            .create_transfer_transaction(
                &sender_address,
                &receiver_address,
                payment.amount as u64, //todo: change the db amount to u64 or i64
                &USDC_MINT.to_string(),
                reference,
            )
            .await?;

        // save the transfer in db
        let transfer_data = TransferModel {
            payment_id: Set(payment.id),
            reference_key: Set(reference_key),
            status: Set(TransferStatus::Pending),
            ..Default::default()
        };

        transfer::Entity::insert(transfer_data)
            .exec_with_returning(state.db())
            .await?;

        // todo: start watching reference key for transaction status

        let serialized_transaction = bincode::serialize(&transfer_transaction)?;
        let base64_transaction = base64::prelude::BASE64_STANDARD.encode(serialized_transaction);

        Ok(base64_transaction)
    }

    pub async fn public_create_transfer(state: AppState, payment_id: i32) -> Result<String> {
        todo!()
    }
}
