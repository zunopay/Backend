use std::sync::Arc;

use sea_orm::EntityTrait;

use crate::{
    db::entity::{prelude::User, user::Model as UserModel},
    services::{
        AppState,
        error::{EntityId, Result, ServiceError},
    },
};

pub mod dto;

pub struct UserService;

impl UserService {
    const USER: &'static str = "User";

    pub async fn find_one(state: Arc<AppState>, id: i32) -> Result<UserModel> {
        let user = User::find_by_id(id).one(state.db()).await?;
        user.ok_or(ServiceError::EntityNotFound {
            entity: Self::USER,
            id: EntityId::Int(id),
        })
    }
}
