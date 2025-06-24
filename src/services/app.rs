use crate::services::error::Result;
use axum::Json;

#[axum::debug_handler]
pub async fn get_healthcheck() -> Result<Json<String>> {
    let response = "Hello World".to_string();
    Ok(Json(response))
}

/*
TEST
1. User register
2. Hardcode wallet for that user
3. Create Payment / Payment cannot be created by unuathorized
4. Create transfer
5. Pay the amount
6. transfer updates to Compeleted, 1% goes to treasury, rest went to user wallet
*/
