use app_lib::{
    commands::activity::{CreateActivityInput, UpdateActivityColumnInput},
    database::activity::{Mutation, Query},
};
use sqlx::SqlitePool;

#[sqlx::test(migrations = "../migrations", fixtures("add_activity.sql"))]
async fn add_activity(db: SqlitePool) {
    let input = CreateActivityInput {
        name: "abc".into(),
        body: Some("def".into()),
        column_id: 1,
    };

    let res = Mutation::create_activity(&db.into(), input).await.unwrap();

    assert_eq!(res.id, 1);
    assert_eq!(res.name, "abc");
    assert_eq!(res.body, Some("def".into()));
    assert_eq!(res.column_id, Some(1));
    assert_eq!(res.ordinal, 0);
}

#[sqlx::test(migrations = "../migrations", fixtures("add_activity_ordinal.sql"))]
async fn add_activity_changes_ordinals(db: SqlitePool) {
    let db = db.into();
    let input = CreateActivityInput {
        name: "abc".into(),
        body: Some("def".into()),
        column_id: 1,
    };

    let res = Mutation::create_activity(&db, input).await.unwrap();

    assert_eq!(res.id, 2);
    assert_eq!(res.name, "abc");
    assert_eq!(res.body, Some("def".into()));
    assert_eq!(res.column_id, Some(1));
    assert_eq!(res.ordinal, 0);

    let all_activities = Query::all_column_activities(&db).await.unwrap();
    assert_eq!(all_activities[&1].name, "Profit");
    assert_eq!(all_activities[&1].ordinal, 1);
}

#[sqlx::test(migrations = "../migrations", fixtures("update_activity_position.sql"))]
async fn update_activity_position_position_unchanged(db: SqlitePool) {
    let db = db.into();

    let all_activities = Query::all_column_activities(&db).await.unwrap();
    assert_eq!(all_activities[&1].ordinal, 0);
    assert_eq!(all_activities[&2].ordinal, 1);
    assert_eq!(all_activities[&3].ordinal, 2);

    Mutation::update_activity_column_by_id(
        &db,
        UpdateActivityColumnInput {
            id: 2,
            column_id: Some(1),
            new_ord: 1,
        },
    )
    .await
    .unwrap();

    let all_activities = Query::all_column_activities(&db).await.unwrap();
    assert_eq!(all_activities[&1].ordinal, 0);
    assert_eq!(all_activities[&2].ordinal, 1);
    assert_eq!(all_activities[&3].ordinal, 2);
}

#[sqlx::test(migrations = "../migrations", fixtures("update_activity_position.sql"))]
async fn update_activity_position_same_column(db: SqlitePool) {
    let db = db.into();

    let all_activities = Query::all_column_activities(&db).await.unwrap();
    assert_eq!(all_activities[&1].ordinal, 0);
    assert_eq!(all_activities[&2].ordinal, 1);
    assert_eq!(all_activities[&3].ordinal, 2);

    Mutation::update_activity_column_by_id(
        &db,
        UpdateActivityColumnInput {
            id: 3,
            column_id: Some(1),
            new_ord: 1,
        },
    )
    .await
    .unwrap();

    let all_activities = Query::all_column_activities(&db).await.unwrap();
    assert_eq!(all_activities[&1].ordinal, 0);
    assert_eq!(all_activities[&2].ordinal, 2);
    assert_eq!(all_activities[&3].ordinal, 1);
}

#[sqlx::test(migrations = "../migrations", fixtures("update_activity_position.sql"))]
async fn update_activity_position_different_column(db: SqlitePool) {
    let db = db.into();

    let all_activities = Query::all_column_activities(&db).await.unwrap();
    assert_eq!(all_activities[&1].ordinal, 0);
    assert_eq!(all_activities[&2].ordinal, 1);
    assert_eq!(all_activities[&3].ordinal, 2);
    assert_eq!(all_activities[&4].ordinal, 0);
    assert_eq!(all_activities[&5].ordinal, 1);
    assert_eq!(all_activities[&6].ordinal, 2);

    Mutation::update_activity_column_by_id(
        &db,
        UpdateActivityColumnInput {
            id: 2,
            column_id: Some(2),
            new_ord: 0,
        },
    )
    .await
    .unwrap();

    let all_activities = Query::all_column_activities(&db).await.unwrap();
    assert_eq!(all_activities[&1].ordinal, 0);
    assert_eq!(all_activities[&2].ordinal, 0);
    assert_eq!(all_activities[&3].ordinal, 1);
    assert_eq!(all_activities[&4].ordinal, 1);
    assert_eq!(all_activities[&5].ordinal, 2);
    assert_eq!(all_activities[&6].ordinal, 3);
}
