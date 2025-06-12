use crate::{
    error::Result,
    services::AppState
};
use axum::{
    Router,
    routing::{get, post},
};

pub fn routes(app_state: AppState) -> Router {
    todo!()
}
