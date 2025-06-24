use std::sync::Arc;

use crate::{
    ctx::mw_resolve_google_ctx::mw_resolve_google_ctx,
    services::{
        AppState,
        auth::{
            AuthService,
            auth_handler::{login, login_with_google, register},
        },
    },
};
use axum::{
    Router, middleware,
    routing::{patch, post},
};

pub fn routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/login-with-google", patch(login_with_google))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            mw_resolve_google_ctx,
        ))
        .route("/register", post(register))
        .route("/login", patch(login))
        .with_state(app_state)
}
