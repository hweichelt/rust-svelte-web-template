use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260909_091320_create_users"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(pk_uuid(Users::Id))
                    // Stored lowercased; uniqueness is enforced on the normalized value.
                    .col(string_uniq(Users::Email))
                    .col(string(Users::DisplayName))
                    .col(string(Users::PasswordHash))
                    .col(timestamp_with_time_zone_default_now(Users::CreatedAt))
                    .col(timestamp_with_time_zone_default_now(Users::UpdatedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Users {
    Table,
    Id,
    Email,
    DisplayName,
    PasswordHash,
    CreatedAt,
    UpdatedAt,
}
