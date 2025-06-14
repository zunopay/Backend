use chrono::{DateTime, NaiveDateTime, Utc};
use serde::Serialize;

use crate::db::entity::{payment::Model as PaymentModel, sea_orm_active_enums::PaymentCategory};

#[derive(Serialize)]
pub struct PaymentDto {
    pub id: i32,

    pub title: String,

    pub description: String,

    pub category: PaymentCategory,

    pub created_at: NaiveDateTime,

    pub amount: i32,
}

pub type PaymentInput = PaymentModel;

impl From<PaymentInput> for PaymentDto {
    fn from(value: PaymentInput) -> Self {
        PaymentDto {
            id: value.id,
            title: value.title,
            description: value.description,
            category: value.category,
            created_at: value.created_at,
            amount: value.amount,
        }
    }
}
