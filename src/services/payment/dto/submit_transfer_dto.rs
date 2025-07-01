use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubmitTransferDto {
    pub payment_id: Uuid,
    pub transaction: String,
}
