use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct SubmitTransferDto {
    pub payment_id: Uuid,
    pub transaction: String,
}
