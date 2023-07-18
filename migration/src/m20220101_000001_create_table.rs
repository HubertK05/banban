use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Activity::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Activity::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Activity::Title).string().not_null())
                    .col(ColumnDef::new(Activity::Body).string().not_null())
                    .to_owned(),
            )
            .await?;

        let db = manager.get_connection();
        let insert = Query::insert().into_table(Activity::Table).columns([Activity::Title, Activity::Body]).values_panic(["Home chores".into(), "Clean the house".into()]).to_owned();
        manager.exec_stmt(insert).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Activity::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Activity {
    Table,
    Id,
    Title,
    Body,
}
