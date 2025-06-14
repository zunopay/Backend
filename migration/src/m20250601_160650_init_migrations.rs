use sea_orm_migration::prelude::{extension::postgres::Type, *};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(User::Email).string().not_null().unique_key())
                    .col(
                        ColumnDef::new(User::Username)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(User::Password).string().not_null())
                    .col(
                        ColumnDef::new(User::S3BucketSlug)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .to_owned(),
            )
            .await?;

        // 1. Create Enum
        manager
            .create_type(
                Type::create()
                    .as_enum(PaymentCategory::Type)
                    .values([PaymentCategory::OneTime])
                    .to_owned(),
            )
            .await?;

        // 2. Create Table
        manager
            .create_table(
                Table::create()
                    .table(Payment::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Payment::Id)
                            .integer()
                            .not_null()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(Payment::Title).string().not_null())
                    .col(ColumnDef::new(Payment::Description).string().not_null())
                    .col(
                        ColumnDef::new(Payment::Category)
                            .custom(PaymentCategory::Type)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Payment::Amount).integer().not_null())
                    .col(
                        ColumnDef::new(Payment::CreatedAt)
                            .not_null()
                            .date_time()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Payment::PublicId)
                            .not_null()
                            .uuid()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Payment::UserId).not_null().integer())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_payment_user_id")
                            .from(Payment::Table, Payment::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(Payment::Table)
                    .name("fk_payment_user_id")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Payment::Table).to_owned())
            .await?;

        manager
            .drop_type(
                Type::drop()
                    .name(PaymentCategory::Type)
                    .if_exists()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Username,
    Email,
    Password,
    S3BucketSlug,
}

#[derive(DeriveIden)]
enum Payment {
    Table,
    Id,
    PublicId,
    Title,
    Description,
    Category,
    CreatedAt,
    Amount,
    UserId,
}

#[derive(DeriveIden)]
enum PaymentCategory {
    #[sea_orm(iden = "payment_category")]
    Type,
    OneTime,
}
