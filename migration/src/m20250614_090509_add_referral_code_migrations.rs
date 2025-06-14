use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .if_not_exists()
                    .table(ReferralCode::Table)
                    .col(
                        ColumnDef::new(ReferralCode::Value)
                            .not_null()
                            .string()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(ReferralCode::Id)
                            .not_null()
                            .integer()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(
                        ColumnDef::new(ReferralCode::RefereeId)
                            .integer()
                            .unique_key(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_referral_code_referee_id")
                            .from(ReferralCode::Table, ReferralCode::RefereeId)
                            .to(User::Table, User::Id),
                    )
                    .col(
                        ColumnDef::new(ReferralCode::ReferrerId)
                            .not_null()
                            .integer(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_referral_code_referrer_id")
                            .from(ReferralCode::Table, ReferralCode::ReferrerId)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(ReferralCode::Table)
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
}

#[derive(DeriveIden)]
enum ReferralCode {
    Table,
    Id,
    Value,
    ReferrerId,
    RefereeId,
}
