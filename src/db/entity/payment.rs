use crate::db::entity::user;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum_macros::EnumString;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "payment")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    pub title: String,

    pub description: String,

    pub category: PaymentCategory,

    pub created_at: DateTimeUtc,

    pub amount: i64,

    pub user_id: i32,
}

#[derive(
    Clone, Debug, PartialEq, EnumIter, DeriveActiveEnum, Serialize, Deserialize, EnumString,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "payment_category")]
pub enum PaymentCategory {
    #[strum(serialize = "OneTime", ascii_case_insensitive)]
    #[sea_orm(string_value = "OneTime")]
    OneTime,
}

// `Relation` enum defines the relationships for this entity (for metadata).
#[derive(Debug, Clone, EnumIter, DeriveRelation)]
pub enum Relation {
    // Here, it defines that this entity (e.g., `Payment`) belongs to `User`.
    #[sea_orm(
        belongs_to="user::Entity"
        from="Column::UserId",
        to="user::Column::Id",
    )]
    User,
}

// Implementing `Related<user::Entity>` allows SeaORM to navigate
// from this entity to `User` using the defined relationship (for functionality).
impl Related<user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
