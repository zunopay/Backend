pub use sea_orm_migration::prelude::*;

mod m20250601_160650_init_migrations;
mod m20250614_041623_transfer_table_migrations;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250601_160650_init_migrations::Migration),
            Box::new(m20250614_041623_transfer_table_migrations::Migration),
        ]
    }
}
