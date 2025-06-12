pub(crate) mod dto;
mod payment_handler;

use super::{
    AppState,
    error::{Result, ServiceError},
};
use crate::{
    ctx::Ctx,
    services::{append_timestamp, user::UserService},
};
use axum::extract::State;
use convert_case::{Case, Casing};
use sea_orm::{
    ActiveValue::Set, EntityName, EntityTrait, TransactionTrait, prelude::DateTimeWithTimeZone,
    sqlx::types::chrono,
};

pub struct PaymentService;

impl PaymentService {
    const TABLE: &'static str = "Payment";

    pub async fn get(state: AppState, id: i32) {
        todo!()
    }

    pub async fn create(
        state: AppState,
        user_id: i32,
    ) {
       todo!()
    }
}
