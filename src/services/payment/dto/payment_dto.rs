use chrono::{DateTime, NaiveDateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::db::entity::{payment::Model as PaymentModel, sea_orm_active_enums::PaymentCategory};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentDto {
    pub id: i32,

    pub public_id: Uuid,

    pub title: String,

    pub description: String,

    pub category: PaymentCategory,

    pub created_at: NaiveDateTime,

    pub amount: u64,
}

pub type PaymentInput = PaymentModel;

impl From<PaymentInput> for PaymentDto {
    fn from(value: PaymentInput) -> Self {
        PaymentDto {
            id: value.id,
            public_id: value.public_id,
            title: value.title,
            description: value.description,
            category: value.category,
            created_at: value.created_at,
            amount: value.amount as u64, // amount will always be postitive
        }
    }
}
