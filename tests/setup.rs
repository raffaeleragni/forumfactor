use axum::Extension;
use axum_test::TestServer;
use forumfactor::app::app;
use velvet_web::prelude::*;

pub async fn setup() -> TestServer {
    JWT::Secret.setup().await.unwrap();
    let db = sqlite().await;
    TestServer::new(app().layer(Extension(db))).unwrap()
}

#[tokio::test]
async fn test_setup() {
    let server = setup().await;
    server.get("/").await.assert_status_ok();
}

#[tokio::test]
async fn test_post() {
    let server = setup().await;
    server.post("/posts").await.assert_status_ok();
}
