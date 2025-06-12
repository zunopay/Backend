pub use sea_orm_migration::prelude::*;

mod m20250601_160650_init_migrations;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20250601_160650_init_migrations::Migration)]
    }
}
