pub mod auth_handler;
pub mod dto;

use crate::config;
use crate::config::config;
use crate::db::entity::{
    prelude::User,
    user::{self, Column, Model as UserModel},
};
use crate::services::auth::dto::authorization_dto::{AuthorizationDto, BasicUserPayload};
use crate::services::auth::dto::login_dto::LoginDto;
use crate::services::auth::dto::{authorization_dto::Claims, register_dto::RegisterDto};
use crate::services::error::{Result, ServiceError};
use crate::services::{AppState, append_timestamp, hash_password, verify_password};
use axum::extract::State;
use chrono;
use jsonwebtoken::{EncodingKey, Header, encode};
use sea_orm::ActiveValue::Set;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, Iden, QueryFilter, QuerySelect};
use tokio::try_join;
use validator::ValidateEmail;

pub struct AuthService;

impl AuthService {
    pub async fn login_with_google(state: AppState, email: String) -> Result<AuthorizationDto> {
        let user = User::find()
            .filter(Column::Email.eq(&email))
            .one(state.db())
            .await?;

        if let Some(val) = user {
            let auth_token = Self::generate_auth_token(val).await?;

            Ok(AuthorizationDto { auth_token })
        } else {
            Self::register_with_google(state, email).await
        }
    }

    async fn register_with_google(state: AppState, email: String) -> Result<AuthorizationDto> {
        let username = Self::generate_username();

        let slug = append_timestamp(&username);
        let s3_bucket_slug = Self::get_s3_bucket(slug);

        let data = user::ActiveModel {
            username: Set(username),
            email: Set(email),
            password: Set("".to_string()),
            s3_bucket_slug: Set(s3_bucket_slug),
            ..Default::default()
        };

        // todo: generate wallet and add in the table

        let user = User::insert(data).exec_with_returning(state.db()).await?;
        let auth_token = Self::generate_auth_token(user).await?;

        Ok(AuthorizationDto { auth_token })
    }

    pub async fn register(state: AppState, body: RegisterDto) -> Result<AuthorizationDto> {
        let username = body.username;
        let email = body.email;

        try_join!(
            Self::check_email(&email, state.db()),
            Self::check_username(&username, state.db())
        )?;

        let slug = append_timestamp(&username);
        let s3_bucket_slug = Self::get_s3_bucket(slug);

        let hashed_password = hash_password(body.password)?;
        let data = user::ActiveModel {
            username: Set(username),
            email: Set(email),
            password: Set(hashed_password),
            s3_bucket_slug: Set(s3_bucket_slug),
            ..Default::default()
        };

        let user = User::insert(data).exec_with_returning(state.db()).await?;

        let auth_token = Self::generate_auth_token(user).await?;

        Ok(AuthorizationDto { auth_token })
    }

    pub async fn login(state: AppState, body: LoginDto) -> Result<AuthorizationDto> {
        let column = if ValidateEmail::validate_email(&body.username_or_email) {
            Column::Email
        } else {
            Column::Username
        };

        let user = User::find()
            .column(Column::Username)
            .one(state.db())
            .await?;

        let user = user.ok_or(ServiceError::UserNotFound)?;
        verify_password(body.password, user.password.clone())?;

        let auth_token = Self::generate_auth_token(user).await?;
        Ok(AuthorizationDto { auth_token })
    }

    pub async fn generate_auth_token(user: UserModel) -> Result<String> {
        let now = chrono::Utc::now();
        let claims = Claims {
            user: BasicUserPayload {
                user_id: user.id,
                email: user.email,
                username: user.username,
            },
            iat: now.timestamp() as usize,
            exp: (now + chrono::Duration::days(7)).timestamp() as usize,
        };

        let auth_token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(config().ACCESS_SECRET_KEY.as_bytes()),
        )?;

        Ok(auth_token)
    }

    async fn check_email(email: &String, db: &DatabaseConnection) -> Result<()> {
        let user = User::find()
            .filter(user::Column::Email.eq(email))
            .one(db)
            .await?;

        if user.is_none() {
            Ok(())
        } else {
            return Err(ServiceError::EmailAlreadyExists);
        }
    }

    async fn check_username(username: &String, db: &DatabaseConnection) -> Result<()> {
        let user = User::find()
            .filter(user::Column::Username.eq(username))
            .one(db)
            .await?;

        if user.is_none() {
            Ok(())
        } else {
            return Err(ServiceError::UsernameAlreadyExists);
        }
    }

    fn get_s3_bucket(user_slug: String) -> String {
        return format!("user/{}", user_slug);
    }

    fn generate_username() -> String {
        todo!()
    }
}
