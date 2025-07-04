use crate::ctx::mw_require_auth::mw_require_auth;
use crate::services::payment::payment_handler::find_one;
use crate::services::user::user_handler::{
    create_merchant_profile, find_all_merchants, find_me, find_merchant,
};
use crate::{error::Result, services::AppState};
use axum::middleware;
use axum::{
    Router,
    routing::{get, post},
};
use std::sync::Arc;

pub fn routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/get/me", get(find_me))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            mw_require_auth,
        ))
        .route("/get-merchant/{slug}", get(find_merchant))
        .route("/get-merchant", get(find_all_merchants))
        .route("/merchant-profile", post(create_merchant_profile))
        .with_state(app_state)
}
