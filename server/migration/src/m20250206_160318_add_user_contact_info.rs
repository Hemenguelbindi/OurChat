use sea_orm_migration::{prelude::*, schema::*};

use crate::m20220101_000001_create_table::{Friend, User};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserContactInfo::Table)
                    .if_not_exists()
                    .col(big_unsigned(UserContactInfo::UserId))
                    .col(big_unsigned(UserContactInfo::ContactUserId))
                    .col(string_len_null(UserContactInfo::DisplayName, 200))
                    .primary_key(
                        Index::create()
                            .col(UserContactInfo::UserId)
                            .col(UserContactInfo::ContactUserId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(UserContactInfo::Table, UserContactInfo::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(UserContactInfo::Table, UserContactInfo::ContactUserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(Friend::Table)
                    .drop_column(Friend::DisplayName)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserContactInfo::Table).to_owned())
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(Friend::Table)
                    .add_column(string_null(Friend::DisplayName))
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum UserContactInfo {
    Table,
    UserId,
    ContactUserId,
    DisplayName,
}
