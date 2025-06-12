use crate::{
    ctx::Ctx,
    services::{
        AppState,
        error::Result,
    },
};
use axum::{
    Json,
    extract::{Multipart, Path, State},
};
use serde_json::json;

pub async fn find_one(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) {
    todo!()
}

pub async fn create(
    State(state): State<AppState>,
    ctx: Ctx,
){
    todo!()
}
