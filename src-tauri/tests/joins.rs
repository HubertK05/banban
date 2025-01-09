use entity::{activities, columns};
use sea_orm::sea_query::Expr;
use sea_orm::{
    DatabaseBackend, DbBackend, EntityTrait, FromQueryResult, JoinType, MockDatabase, QuerySelect,
    QueryTrait, RelationTrait,
};

#[derive(FromQueryResult, Debug)]
struct JoinQueryRes {
    activity_id: Option<i32>,
    body: Option<String>,
    column_id: Option<i32>,
    name: String,
    // column_name: Option<String>,
}

#[tokio::test]
async fn mock_interface_example() {
    let db = MockDatabase::new(DatabaseBackend::Sqlite)
        .append_query_results([vec![
            activities::Model {
                id: 1,
                name: "test activity 1".to_string(),
                body: Some("body once told me".to_string()),
                column_id: Some(1),
                ordinal: 0,
            },
            activities::Model {
                id: 2,
                name: "test activity 2".to_string(),
                body: None,
                column_id: None,
                ordinal: 0,
            },
        ]])
        .append_query_results([vec![
            columns::Model {
                id: 1,
                name: "to do".to_string(),
                ordinal: 0,
            },
            columns::Model {
                id: 2,
                name: "done".to_string(),
                ordinal: 0,
            },
        ]])
        .into_connection();

    // this type of join forces us to use the current Model, but lets us choose:
    // - the type of join,
    // - the relation
    let res1 = activities::Entity::find()
        .select_only()
        .column_as(activities::Column::Id, "activity_id")
        // .column_as(activities::Column::Name, "name")
        .column_as(activities::Column::Body, "body")
        .column_as(activities::Column::ColumnId, "column_id")
        // .column_as(activities::Column::Id, "column_id")
        .column_as(columns::Column::Name, "name")
        .join(JoinType::InnerJoin, activities::Relation::Columns.def())
        // .build(DbBackend::Sqlite).to_string();
        .into_model::<JoinQueryRes>()
        .all(&db)
        .await;

    let res2: Result<Option<activities::Model>, sea_orm::DbErr> = activities::Entity::find_by_id(1)
        .join_rev(JoinType::InnerJoin, columns::Relation::Activities.def())
        // .into_model::<JoinQueryRes>()
        .one(&db)
        .await;

    println!("{res1:?}");
    println!("{res2:?}");
    panic!();

    // assert_eq!(
    //     res1.unwrap(),
    //     Some(activities::Model {
    //         id: 1,
    //         name: "Test database".to_owned(),
    //         body: None,
    //         column_id: None,
    //     })
    // );
}
