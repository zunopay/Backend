use crate::{ctx::CtxResult, error::Result};
use axum::{extract::Request, middleware::Next, response::Response};

pub async fn mw_require_auth(ctx: CtxResult, request: Request, next: Next) -> Result<Response> {
    ctx?;

    Ok(next.run(request).await)
}
