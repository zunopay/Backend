use crate::services::{
    AppState,
    auth::{
        AuthService,
        auth_handler::{login, register},
    },
};
use axum::{
    Router,
    routing::{patch, post},
};

pub fn routes(app_state: AppState) -> Router {
    let router = Router::new()
        .route("/register", post(register))
        .route("/login", patch(login))
        .with_state(app_state);

    router
}
