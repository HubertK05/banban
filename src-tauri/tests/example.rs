use entity::activities;
use sea_orm::{DatabaseBackend, EntityTrait, MockDatabase};

#[tokio::test]
async fn mock_interface_example() {
    // Create MockDatabase with mock query results
    let db = MockDatabase::new(DatabaseBackend::Sqlite)
        .append_query_results([vec![activities::Model {
            id: 1,
            name: "Test database".to_string(),
            body: None,
            column_id: None,
            ordinal: 0,
        }]])
        .into_connection();

    assert_eq!(
        activities::Entity::find_by_id(1).one(&db).await.unwrap(),
        Some(activities::Model {
            id: 1,
            name: "Test database".to_owned(),
            body: None,
            column_id: None,
            ordinal: 0,
        })
    );
}
