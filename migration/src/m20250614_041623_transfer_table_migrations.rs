use sea_orm_migration::prelude::{extension::postgres::Type, *};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 1. Create Enum
        manager
            .create_type(
                Type::create()
                    .as_enum(TransferStatus::Type)
                    .values([
                        TransferStatus::Pending,
                        TransferStatus::Completed,
                        TransferStatus::Rejected,
                    ])
                    .to_owned(),
            )
            .await?;

        // 2. Create Table
        manager
            .create_table(
                Table::create()
                    .table(Transfer::Table)
                    .col(
                        ColumnDef::new(Transfer::Id)
                            .integer()
                            .auto_increment()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Transfer::ReferenceKey)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Transfer::PaymentId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_transfer_payment_id")
                            .from(Transfer::Table, Transfer::PaymentId)
                            .to(Payment::Table, Payment::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(Transfer::Signature).string())
                    .col(
                        ColumnDef::new(Transfer::Status)
                            .custom(TransferStatus::Type)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Transfer::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
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
                    .table(Transfer::Table)
                    .name("fk_transfer_payment_id")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(Transfer::Table).if_exists().to_owned())
            .await?;

        manager
            .drop_type(
                Type::drop()
                    .if_exists()
                    .name(TransferStatus::Type)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Transfer {
    Table,
    Id,
    ReferenceKey,
    PaymentId,
    CreatedAt,
    Signature,
    Status,
}

#[derive(DeriveIden)]
enum TransferStatus {
    #[sea_orm(iden = "transfer_status")]
    Type,
    Pending,
    Completed,
    Rejected,
}

#[derive(DeriveIden)]
enum Payment {
    Table,
    Id,
}
