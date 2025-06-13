use crate::db::entity::payment::PaymentCategory;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreatePaymentDto {
    pub title: String,

    pub description: String,

    pub category: PaymentCategory,

    pub amount: i64,
}
