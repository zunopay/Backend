use sea_orm_migration::prelude::{extension::postgres::Type, *};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(MerchantCategory::Type)
                    .values([
                        MerchantCategory::Restaurant,
                        MerchantCategory::Grocerries,
                        MerchantCategory::Other,
                    ])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Merchant::Table)
                    .col(
                        ColumnDef::new(Merchant::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Merchant::DisplayName).string().not_null())
                    .col(
                        ColumnDef::new(Merchant::Slug)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Merchant::Cover).string())
                    .col(ColumnDef::new(Merchant::Address).string().not_null())
                    .col(
                        ColumnDef::new(Merchant::Category)
                            .custom(MerchantCategory::Type)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Merchant::S3BucketSlug).string().not_null())
                    .col(ColumnDef::new(Merchant::BusinessRegistrationNumber).string())
                    .col(
                        ColumnDef::new(Merchant::IsVerified)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Merchant::UserId)
                            .integer()
                            .not_null()
                            .unique_key(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_merchant_user_id")
                            .from(Merchant::Table, Merchant::UserId)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Merchant::Table).if_exists().to_owned())
            .await?;

        manager
            .drop_type(
                Type::drop()
                    .if_exists()
                    .name(MerchantCategory::Type)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Merchant {
    Table,
    Id,
    DisplayName,
    Slug,
    Cover,
    S3BucketSlug,
    Address,
    BusinessRegistrationNumber,
    IsVerified,
    UserId,
    Category,
}

#[derive(DeriveIden)]
enum MerchantCategory {
    #[sea_orm(iden = "merchant_category")]
    Type,
    Restaurant,
    Grocerries,
    Other,
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
}
