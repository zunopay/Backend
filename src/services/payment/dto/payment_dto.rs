use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::db::entity::{
    Payment,
    payment::{PaymentCategory, PaymentCategoryEnum},
};

#[derive(Serialize)]
pub struct PaymentDto {
    pub id: i32,

    pub title: String,

    pub description: String,

    pub category: PaymentCategory,

    pub created_at: DateTime<Utc>,

    pub amount: i64,
}

pub type PaymentInput = Payment;

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
