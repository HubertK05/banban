use sea_orm::{DatabaseBackend, MockDatabase, EntityTrait};
use entity::activities;

#[tokio::test]
async fn mock_interface_example() {
    // Create MockDatabase with mock query results
    let db = MockDatabase::new(DatabaseBackend::Sqlite)
        .append_query_results([
            vec![activities::Model {
                id: 1,
                name: "Test database".to_string(),
                body: None,
                column_id: None,
            }]
        ])
        .into_connection();

    assert_eq!(
        activities::Entity::find_by_id(1).one(&db).await.unwrap(),
        Some(activities::Model {
            id: 1,
            name: "Test database".to_owned(),
            body: None,
            column_id: None,
        })
    );
}
