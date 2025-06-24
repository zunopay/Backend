use crate::ctx::mw_require_auth::mw_require_auth;
use crate::services::payment::payment_handler::{create, create_transfer, find_one};
use crate::{error::Result, services::AppState};
use axum::middleware;
use axum::{
    Router,
    routing::{get, post},
};
use std::sync::Arc;

pub fn routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/create", post(create))
        .route("/get/{id}", get(find_one))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            mw_require_auth,
        ))
        .route("/create-transfer", post(create_transfer))
        .with_state(app_state)
}

/*
 * 1. Payment directly on the website
 * 2. Payment gateway integeration with the client website
 */
