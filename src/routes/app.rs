use crate::{error::Result, services::app::get_healthcheck};
use axum::{Json, Router, response::IntoResponse, routing::get};

pub fn routes() -> Router {
    let router = Router::new().route("/", get(get_healthcheck));
    router
}
