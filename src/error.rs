use std::sync::Arc;

use axum::{Json, body::Body, response::IntoResponse};
use reqwest::StatusCode;
use strum_macros::AsRefStr;

use crate::services;

pub type Result<T> = std::result::Result<T, Error>;

//Might migrate to String errors instead using Arc
#[derive(Debug, AsRefStr, Clone)]
pub enum Error {
    EnvMissing(&'static str),
    FailedCtxErrorNotInRequestExtension,
    MissingAuthToken,
    JwtError(jsonwebtoken::errors::Error),
    DatabaseError(Arc<sea_orm::error::DbErr>),
    ServiceError(String),
    MiddlewareError(&'static str),
    FailedToBindListener { port: i32, e: String },
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        dbg!(self.clone());
        write!(f, "{}", &self.as_ref())
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        Error::JwtError(value)
    }
}

impl From<services::error::ServiceError> for Error {
    fn from(value: services::error::ServiceError) -> Self {
        Error::ServiceError(value.to_string())
    }
}

impl From<sea_orm::error::DbErr> for Error {
    fn from(value: sea_orm::error::DbErr) -> Self {
        Error::DatabaseError(Arc::new(value))
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        dbg!(self.clone());
        let status = StatusCode::BAD_REQUEST;
        let body = Json(self.to_string());

        (status, body).into_response()
    }
}
