use axum::Extension;
use axum_test::TestServer;
use forumfactor::app::app;
use velvet_web::prelude::*;

pub async fn setup() -> TestServer {
    let db = newdb().await;
    JWT::Secret.setup().await.unwrap();

    TestServer::new(app().layer(Extension(db))).unwrap()
}

async fn newdb() -> Pool<Sqlite> {
    std::fs::remove_file("test.db").unwrap_or(());
    let db = sqlite().await;
    sqlx::migrate!().run(&db).await.unwrap();
    db
}

#[tokio::test]
async fn test_setup() {
    let server = setup().await;
    server.get("/").await.assert_status_ok();
}

#[tokio::test]
async fn test_post() {
    #[derive(Serialize)]
    struct Form<'a> {
        title: &'a str,
        post: &'a str,
    }
    let f = Form {
        title: "posted title",
        post: "posted post",
    };

    let server = setup().await;
    server.post("/topics").form(&f).await.assert_status_ok();

    server
        .get("/topics")
        .await
        .assert_text_contains("posted title");
    server
        .get("/posts")
        .await
        .assert_text_contains("posted post");
}
