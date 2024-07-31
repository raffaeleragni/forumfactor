use axum::Extension;
use axum_test::TestServer;
use forumfactor::app::app;
use serde::{Deserialize, Serialize};
use velvet_web::prelude::{sqlite, JWT};

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
async fn test_post_topic() {
    #[derive(Serialize)]
    struct Form {
        title: String,
        text: String,
    }
    #[derive(Deserialize)]
    struct Response {
        id: String,
        title: Option<String>,
        text: Option<String>
    }
    let form = Form {
        title: "title".into(),
        text: "post".into(),
    };

    let server = setup().await;
    let post = server.post("/topic").form(&form).await;
    post.assert_status_ok();
    let post_id = post.json::<Response>().id;

    let post = server.get(format!("/topic/{}", post_id).as_str()).await;
    post.assert_status_ok();
    let response = post.json::<Response>();
}
