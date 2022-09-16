use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_index(
                sea_query::Index::create()
                .name("idx_user_email")
                .table(User::Table)
                .col(User::Email).to_owned()
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(sea_query::Index::drop()
                .name("idx_user_email")
                .table(User::Table)
                .to_owned()
            )
            .await
    }
}

#[derive(Iden)]
enum User {
    Table,
    Email
}