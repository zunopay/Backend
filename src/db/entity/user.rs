use crate::db::entity::payment;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    #[sea_orm(unique, column_type = "Text")]
    pub username: String,

    #[sea_orm(unique, column_type = "Text")]
    pub email: String,

    #[sea_orm(column_type = "Text")]
    pub password: String,

    #[sea_orm(unique)]
    pub s3_bucket_slug: String,
}

#[derive(Clone, Copy, Debug, DeriveRelation, EnumIter)]
pub enum Relation {
    #[sea_orm(has_many = "payment::Entity")]
    Payment,
}

impl Related<payment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Payment.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
