use std::sync::Arc;

use convert_case::{Case, Casing};
use sea_orm::{ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};

use crate::{
    ctx::Ctx,
    db::entity::{
        merchant::{self, Column, Model as MerchantModel},
        prelude::{Merchant, User},
        user::Model as UserModel,
    },
    services::{
        AppState,
        error::{EntityId, Result, ServiceError},
        user::dto::create_merchant_profile_dto::CreateMerchantProfileDto,
    },
};

pub mod dto;

pub struct UserService;

impl UserService {
    const USER: &'static str = "User";
    const MERCHANT: &'static str = "Merchant";

    pub async fn find_one(state: Arc<AppState>, id: i32) -> Result<UserModel> {
        let user = User::find_by_id(id).one(state.db()).await?;
        user.ok_or(ServiceError::EntityNotFound {
            entity: Self::USER,
            id: EntityId::Int(id),
        })
    }

    pub async fn find_merchant(state: Arc<AppState>, slug: String) -> Result<MerchantModel> {
        let merchant = Merchant::find()
            .filter(Column::Slug.eq(&slug))
            .one(state.db())
            .await?;

        merchant.ok_or(ServiceError::EntityNotFound {
            entity: Self::MERCHANT,
            id: EntityId::Str(slug),
        })
    }

    pub async fn find_all_merchants(state: Arc<AppState>) -> Result<Vec<MerchantModel>> {
        let merchants = Merchant::find().all(state.db()).await?;
        Ok(merchants)
    }

    pub async fn create_merchant_profile(
        state: Arc<AppState>,
        ctx: Ctx,
        create_merchant_profile_dto: CreateMerchantProfileDto,
    ) -> Result<MerchantModel> {
        let user = Self::find_one(state.clone(), ctx.user_id).await?;
        let display_name = create_merchant_profile_dto.display_name;
        let slug = display_name.to_case(Case::Kebab);
        let s3_bucket_slug = Self::get_merchant_s3_bucket(&user.username, &slug);

        let cover = match create_merchant_profile_dto.cover {
            Some(cover) => Some(
                state
                    .s3
                    .upload_file(&s3_bucket_slug, &cover, Some("cover".to_string()))
                    .await?,
            ),
            None => None,
        };

        let data = merchant::ActiveModel {
            display_name: Set(display_name),
            address: Set(create_merchant_profile_dto.address),
            business_registration_number: Set(
                create_merchant_profile_dto.business_registration_number
            ),
            slug: Set(slug),
            s3_bucket_slug: Set(s3_bucket_slug),
            cover: Set(cover),
            ..Default::default()
        };

        let merchat = Merchant::insert(data)
            .exec_with_returning(state.db())
            .await?;
        Ok(merchat)
    }

    fn get_merchant_s3_bucket(username: &String, merchant_slug: &String) -> String {
        return format!("user/{}/merchant/{}", username, merchant_slug);
    }
}
