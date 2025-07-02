use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
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
                    .col(ColumnDef::new(Merchant::S3BucketSlug).string().not_null())
                    .col(ColumnDef::new(Merchant::BusinessRegistrationNumber).string())
                    .col(
                        ColumnDef::new(Merchant::IsVerified)
                            .boolean()
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
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
}
