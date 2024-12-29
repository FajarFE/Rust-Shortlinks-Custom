use loco_rs::schema::table_auto_tz;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto_tz(Domains::Table)
                    .col(pk_auto(Domains::Id))
                    .col(string_null(Domains::Domain))
                    .col(string_null(Domains::Status))
                    .col(integer(Domains::UsersId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-domains-users")
                            .from(Domains::Table, Domains::UsersId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Domains::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Domains {
    Table,
    Id,
    Domain,
    Status,
    UsersId,
    
}


#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
