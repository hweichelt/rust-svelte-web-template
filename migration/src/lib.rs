pub use sea_orm_migration::prelude::*;

mod m20260909_091320_create_users;
mod m20260909_091321_create_sessions;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260909_091320_create_users::Migration),
            Box::new(m20260909_091321_create_sessions::Migration),
        ]
    }
}
