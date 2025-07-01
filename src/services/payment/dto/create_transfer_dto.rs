use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTransferDto {
    pub sender_address: String,

    pub payment_id: Uuid,
}
