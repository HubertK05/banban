use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Column::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Column::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Column::Name).string().not_null())
                    .to_owned(),
            )
            .await?;

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
                    .col(ColumnDef::new(Activity::ColumnId).integer())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_activity_id")
                            .from(Activity::Table, Activity::ColumnId)
                            .to(Column::Table, Column::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Category::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Category::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Category::Name).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Tag::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Tag::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Tag::Name).string().not_null())
                    .col(ColumnDef::new(Tag::CategoryID).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Tag::Table, Tag::CategoryID)
                            .to(Category::Table, Category::Id),
                    )
                    .to_owned(),
            )
            .await?;

        let column_id = 1;
        let insert = Query::insert()
            .into_table(Column::Table)
            .columns([Column::Id, Column::Name])
            .values_panic([column_id.into(), "New".into()])
            .to_owned();
        manager.exec_stmt(insert).await?;

        let insert = Query::insert()
            .into_table(Activity::Table)
            .columns([Activity::Title, Activity::Body, Activity::ColumnId])
            .values_panic([
                "Homework".into(),
                "Workbook page 31".into(),
                column_id.into(),
            ])
            .values_panic([
                "Home chores".into(),
                "Clean the house".into(),
                column_id.into(),
            ])
            .to_owned();
        manager.exec_stmt(insert).await?;

        let insert = Query::insert()
            .into_table(Category::Table)
            .columns([Category::Id, Category::Name])
            .values_panic([1.into(), "Size".into()])
            .to_owned();
        manager.exec_stmt(insert).await?;

        let insert = Query::insert()
            .into_table(Tag::Table)
            .columns([Tag::Id, Tag::Name, Tag::CategoryID])
            .values_panic([1.into(), "small".into(), 1.into()])
            .values_panic([2.into(), "medium".into(), 1.into()])
            .values_panic([3.into(), "big".into(), 1.into()])
            .to_owned();
        manager.exec_stmt(insert).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Activity::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Column::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Tag::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Category::Table).to_owned())
            .await?;
        Ok(())
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Activity {
    Table,
    Id,
    Title,
    Body,
    ColumnId,
}

#[derive(Iden)]
enum Column {
    Table,
    Id,
    Name,
}

#[derive(Iden)]
enum Category {
    Table,
    Id,
    Name,
}

#[derive(Iden)]
enum Tag {
    Table,
    Id,
    Name,
    CategoryID,
}
