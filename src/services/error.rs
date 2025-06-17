use std::fmt::Display;

use aws_sdk_s3::operation::put_object::PutObjectError;
use axum::{
    Json,
    extract::multipart::MultipartError,
    http::StatusCode,
    response::{IntoResponse, IntoResponseParts},
};
use strum_macros::AsRefStr;

pub type Result<T> = std::result::Result<T, ServiceError>;

#[derive(Debug, Clone, AsRefStr)]
pub enum EntityId {
    Int(i32),
    Str(String),
}

#[derive(Debug, AsRefStr, Clone)]
pub enum MathErrorType {
    NumericalOverflow,
}

#[derive(Debug, AsRefStr, Clone)]
pub enum ServiceError {
    EntityNotFound { entity: &'static str, id: EntityId },
    Database(String),
    UserNotFound,
    InvalidPassword,
    DtoError(String),
    ParseError(strum::ParseError),
    JwtError(jsonwebtoken::errors::Error),
    EmailAlreadyExists,
    UsernameAlreadyExists,
    PasswordHashError(argon2::password_hash::Error),
    ValidationError(validator::ValidationErrors),
    S3Error(String),
    Web3Error(String),
    SerializationError(String),
    KeypairError(String),
    MathError(MathErrorType),
}

impl std::fmt::Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.as_ref())
    }
}

impl std::error::Error for ServiceError {}

impl From<sea_orm::DbErr> for ServiceError {
    fn from(value: sea_orm::DbErr) -> Self {
        Self::Database(value.to_string())
    }
}

impl IntoResponse for ServiceError {
    fn into_response(self) -> axum::response::Response {
        /*
           TODO:
           1. Customize status code for each error
           2. Change to client side error
        */
        dbg!(self.clone());
        let status = StatusCode::BAD_REQUEST;
        let body = Json(self.to_string());

        // (T,S) : T should implement IntoResponseParts and S,T implements IntoResponse
        (status, body).into_response()
    }
}

impl From<MultipartError> for ServiceError {
    fn from(value: MultipartError) -> Self {
        Self::DtoError(value.body_text())
    }
}

impl From<strum::ParseError> for ServiceError {
    fn from(value: strum::ParseError) -> Self {
        Self::ParseError(value)
    }
}

impl From<jsonwebtoken::errors::Error> for ServiceError {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        Self::JwtError(value)
    }
}

impl From<argon2::password_hash::Error> for ServiceError {
    fn from(value: argon2::password_hash::Error) -> Self {
        Self::PasswordHashError(value)
    }
}

impl From<validator::ValidationErrors> for ServiceError {
    fn from(value: validator::ValidationErrors) -> Self {
        Self::ValidationError(value)
    }
}

impl From<bincode::Error> for ServiceError {
    fn from(value: bincode::Error) -> Self {
        Self::SerializationError(value.to_string())
    }
}

impl From<solana_client::client_error::ClientError> for ServiceError {
    fn from(value: solana_client::client_error::ClientError) -> Self {
        Self::Web3Error(value.to_string())
    }
}

/**
 * Convert each sdk error types: PutObjectError, GetObjectError ..
 * - All must implement Display + Send + Sync + 'static for thread safe and static lifetime
 */
impl<E> From<aws_sdk_s3::error::SdkError<E>> for ServiceError
where
    E: Display + Send + Sync + 'static,
{
    fn from(value: aws_sdk_s3::error::SdkError<E>) -> Self {
        Self::S3Error(value.to_string())
    }
}

impl From<spl_token::solana_program::program_error::ProgramError> for ServiceError {
    fn from(value: spl_token::solana_program::program_error::ProgramError) -> Self {
        Self::Web3Error(value.to_string())
    }
}

impl From<spl_associated_token_account::solana_program::pubkey::ParsePubkeyError> for ServiceError {
    fn from(value: spl_associated_token_account::solana_program::pubkey::ParsePubkeyError) -> Self {
        Self::Web3Error(value.to_string())
    }
}
