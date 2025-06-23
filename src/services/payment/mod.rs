pub(crate) mod dto;
pub mod payment_handler;

use std::sync::Arc;

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
        error::{EntityId, MathErrorType},
        indexer::Indexer,
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

    pub async fn find_one(state: Arc<AppState>, id: i32) -> Result<PaymentInput> {
        let payment = Payment::find_by_id(id).one(state.db()).await?;
        payment.ok_or(ServiceError::EntityNotFound {
            entity: Self::TABLE,
            id: EntityId::Int(id),
        })
    }

    pub async fn create(
        state: Arc<AppState>,
        user_id: i32,
        create_payment_dto: CreatePaymentDto,
    ) -> Result<PaymentInput> {
        let amount = i64::try_from(create_payment_dto.amount)
            .map_err(|_| ServiceError::MathError(MathErrorType::NumericalOverflow))?;

        let data = payment::ActiveModel {
            title: Set(create_payment_dto.title),
            description: Set(create_payment_dto.description),
            amount: Set(amount),
            created_at: Set(Utc::now().naive_utc()),
            category: Set(create_payment_dto.category),
            public_id: Set(Uuid::new_v4()),
            user_id: Set(user_id),
            ..Default::default()
        };

        let payment = Payment::insert(data)
            .exec_with_returning(state.db())
            .await?;
        Ok(payment)
    }

    pub async fn create_transfer(
        state: Arc<AppState>,
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

        // todo: validate the transfer (allowlist or other criteria's)

        // create transfer tx
        let amount = u64::try_from(payment.amount)
            .map_err(|_| ServiceError::MathError(MathErrorType::NumericalOverflow))?;
        let transfer_transaction = state
            .web3
            .create_transfer_transaction(
                &sender_address,
                &receiver_address,
                amount,
                &USDC_MINT.to_string(),
                reference,
            )
            .await?;

        // save the transfer in db
        let transfer_data = TransferModel {
            payment_id: Set(payment.id),
            reference_key: Set(reference_key.clone()),
            status: Set(TransferStatus::Pending),
            sender_wallet_address: Set(sender_address),
            ..Default::default()
        };

        transfer::Entity::insert(transfer_data)
            .exec_with_returning(state.db())
            .await?;

        // TODO: Use timer wheel algorithm for indexing
        let mint = USDC_MINT;
        tokio::spawn(async move {
            let result = Indexer::poll_payment(
                state,
                reference_key,
                receiver_address,
                mint.to_string(),
                amount,
            )
            .await;

            if let Err(e) = result {
                println!("poll_payment failed: {:?}", e); // or use tracing::error!
            }
        });

        let serialized_transaction = bincode::serialize(&transfer_transaction)?;
        let base64_transaction = base64::prelude::BASE64_STANDARD.encode(serialized_transaction);

        Ok(base64_transaction)
    }

    pub async fn public_create_transfer(state: AppState, payment_id: i32) -> Result<String> {
        todo!()
    }
}
