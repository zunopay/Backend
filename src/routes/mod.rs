pub mod app;
pub mod auth;
pub mod payment;
pub mod user;

use crate::{
    ctx::{mw_require_auth::mw_require_auth, mw_resolve_ctx::mw_resolve_ctx},
    error::Result,
    services::AppState,
};
use axum::{Extension, Router, middleware};
use std::sync::Arc;
use tower_cookies::CookieManagerLayer;

pub async fn routes() -> Result<Router> {
    let app_state = Arc::new(AppState::new().await?);
    let router = Router::new()
        .nest("/payment", payment::routes(app_state.clone()))
        .nest("/auth", auth::routes(app_state.clone()))
        .merge(app::routes())
        .layer(middleware::from_fn_with_state(app_state, mw_resolve_ctx))
        .layer(CookieManagerLayer::new());

    Ok(router)
}
