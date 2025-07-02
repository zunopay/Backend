use crate::services::error::Result;
use axum::Json;

#[axum::debug_handler]
pub async fn get_healthcheck() -> Result<Json<String>> {
    let response = "Hello World".to_string();
    Ok(Json(response))
}
