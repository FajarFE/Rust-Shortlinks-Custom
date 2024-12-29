use loco_rs::schema::table_auto_tz;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto_tz(Shortlinks::Table)
                    .col(pk_auto(Shortlinks::Id))
                    .col(string_null(Shortlinks::Shortlink))
                    .col(string_null(Shortlinks::Url))
                    .col(integer_null(Shortlinks::Clicks))
                    .col(integer(Shortlinks::UsersId))
                    .col(integer(Shortlinks::DomainId))
                    .col(string_null(Shortlinks::StatusPublic))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-shortlinks-users")
                            .from(Shortlinks::Table, Shortlinks::UsersId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-shortlinks-domains")
                            .from(Shortlinks::Table, Shortlinks::DomainId)
                            .to(Domains::Table, Domains::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Shortlinks::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Shortlinks {
    Table,
    Id,
    Shortlink,
    Url,
    Clicks,
    UsersId,
    DomainId,
    StatusPublic,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
#[derive(DeriveIden)]
enum Domains {
    Table,
    Id,
}
