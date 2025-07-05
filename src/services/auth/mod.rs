pub mod auth_handler;
pub mod dto;

use std::sync::Arc;

use crate::config;
use crate::config::config;
use crate::constants::PRIVY_BASE_URL;
use crate::db::entity::{
    prelude::User,
    user::{self, Column, Model as UserModel},
};
use crate::services::auth::dto::authorization_dto::{AuthorizationDto, BasicUserPayload};
use crate::services::auth::dto::login_dto::LoginDto;
use crate::services::auth::dto::wallet_dto::{PrivyUser, PrivyWallet};
use crate::services::auth::dto::{authorization_dto::Claims, register_dto::RegisterDto};
use crate::services::error::{Result, ServiceError};
use crate::services::{AppState, append_timestamp, hash_password, verify_password};
use axum::extract::State;
use base64::Engine;
use base64::engine::general_purpose;
use chrono;
use jsonwebtoken::{EncodingKey, Header, encode};
use reqwest::{Body, Client};
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, Iden, QueryFilter, QuerySelect, SelectColumns,
};
use serde_json::json;
use tokio::try_join;
use validator::ValidateEmail;

pub struct AuthService;

impl AuthService {
    pub async fn login_with_google(
        state: Arc<AppState>,
        email: String,
    ) -> Result<AuthorizationDto> {
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

    async fn register_with_google(state: Arc<AppState>, email: String) -> Result<AuthorizationDto> {
        let name = email
            .split('@')
            .next()
            .ok_or(ServiceError::Custom("Invalid email".to_string()))?
            .to_string();

        let slug = append_timestamp(&name);
        let s3_bucket_slug = Self::get_s3_bucket(slug);

        let privy_wallet = Self::get_privy_user_by_email(&email).await?;
        let wallet_address = privy_wallet.map(|val| val.address);

        let data = user::ActiveModel {
            email: Set(email),
            password: Set("".to_string()), // Empty password for oauth users
            s3_bucket_slug: Set(s3_bucket_slug),
            wallet_address: Set(wallet_address),
            ..Default::default()
        };

        let user = User::insert(data).exec_with_returning(state.db()).await?;
        let auth_token = Self::generate_auth_token(user).await?;

        Ok(AuthorizationDto { auth_token })
    }

    pub async fn register(state: Arc<AppState>, body: RegisterDto) -> Result<AuthorizationDto> {
        let email = body.email;

        Self::check_email(&email, state.db()).await?;

        let name = email
            .split('@')
            .next()
            .ok_or(ServiceError::Custom("Invalid email".to_string()))?
            .to_string();

        let slug = append_timestamp(&name);
        let s3_bucket_slug = Self::get_s3_bucket(slug);

        let hashed_password = hash_password(body.password)?;
        let data = user::ActiveModel {
            email: Set(email),
            password: Set(hashed_password),
            s3_bucket_slug: Set(s3_bucket_slug),
            ..Default::default()
        };

        let user = User::insert(data).exec_with_returning(state.db()).await?;

        let auth_token = Self::generate_auth_token(user).await?;

        Ok(AuthorizationDto { auth_token })
    }

    pub async fn login(state: Arc<AppState>, body: LoginDto) -> Result<AuthorizationDto> {
        let user = User::find()
            .filter(Column::Email.eq(body.email))
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

    fn get_s3_bucket(user_slug: String) -> String {
        return format!("user/{}", user_slug);
    }

    async fn get_privy_user_by_email(email: &String) -> Result<Option<PrivyWallet>> {
        let client = Client::new();
        let url = format!("{}/v1/users/email/address", PRIVY_BASE_URL);

        let body = json!({"address": email});

        let app_id = &config().PRIVY_APP_ID;
        let credentials = format!("{}:{}", app_id, config().PRIVY_APP_SECRET);
        let encoded_credentials = general_purpose::STANDARD.encode(credentials);

        let response = client
            .post(url)
            .json(&body)
            .header("Authorization", format!("Basic {}", encoded_credentials))
            .header("privy-app-id", app_id)
            .header("Content-Type", "application/json")
            .send()
            .await?;

        let privy_user = response.json::<PrivyUser>().await?;

        let wallet = privy_user.wallet;
        Ok(wallet)
    }
}
