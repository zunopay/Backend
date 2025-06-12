pub mod mw_require_auth;
pub mod mw_resolve_ctx;

use crate::error::{Error, Result};
use axum::{
    extract::{FromRequestParts, Request},
    http::request::Parts,
};
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct Ctx {
    pub user_id: i32,
}

type CtxResult = core::result::Result<Ctx, Error>;

//async_trait is removed in axum 0.8 for defining asynchronous traits
impl<S> FromRequestParts<S> for Ctx
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> CtxResult {
        parts
            .extensions
            .get::<CtxResult>()
            .ok_or(Error::FailedCtxErrorNotInRequestExtension)?
            .clone()
    }
}

/*
    FromRequestParts: To define extracter without consuming the body
    FromRequest: To define extracter and consumes the body

*/
