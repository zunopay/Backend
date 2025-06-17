use crate::db::entity::sea_orm_active_enums::PaymentCategory;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreatePaymentDto {
    pub title: String,

    pub description: String,

    pub category: PaymentCategory,

    pub amount: u64,
}
