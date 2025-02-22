use app_lib::database::category::Mutation;
use sqlx::SqlitePool;

#[sqlx::test(migrations = "../migrations", fixtures("insert_category.sql"))]
async fn insert_category_appends_to_end(db: SqlitePool) {
    let db = db.into();

    let res = Mutation::insert_category(&db, "test".into()).await.unwrap();
    assert_eq!(res.id, 2);
    assert_eq!(res.name, "test");
    assert_eq!(res.ordinal, 1);
}
