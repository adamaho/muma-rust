use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Restaurants::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Restaurants::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Restaurants::Name).string().not_null())
                    .col(ColumnDef::new(Restaurants::AddressLine1).string_len(1000))
                    .col(ColumnDef::new(Restaurants::AddressLine2).string_len(1000))
                    .col(ColumnDef::new(Restaurants::City).string())
                    .col(ColumnDef::new(Restaurants::Country).string())
                    .col(ColumnDef::new(Restaurants::PostalCode).string())
                    .col(ColumnDef::new(Restaurants::State).string())
                    .col(
                        ColumnDef::new(Restaurants::CreatedAt)
                            .timestamp()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp))
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Restaurants::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Restaurants {
    Table,
    #[iden = "restaurant_id"]
    Id,
    Name,
    AddressLine1,
    AddressLine2,
    City,
    Country,
    PostalCode,
    State,
    CreatedAt,
}
