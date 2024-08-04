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
    let server = setup().await;
    let topic_id = server.make_post("posted title", "posted post").await;

    server
        .get("/topics")
        .await
        .assert_text_contains("posted title");
    server
        .get(format!("/posts/{topic_id}").as_str())
        .await
        .assert_text_contains("posted post");
}

trait MakePost {
    async fn make_post<'a>(&self, title: &'a str, post: &'a str) -> i64;
}

trait MakeReply {
    async fn make_reply<'a>(&self, topic_id: i64, post: &'a str);
}

impl MakePost for TestServer {
    async fn make_post<'a>(&self, title: &'a str, post: &'a str) -> i64 {
        #[derive(Serialize)]
        struct Form<'a> {
            title: &'a str,
            post: &'a str,
        }
        let f = Form { title, post };

        let response = self.post("/topics").form(&f).await;
        response.assert_status_ok();
        response
            .header("ID")
            .to_str()
            .unwrap()
            .parse::<i64>()
            .unwrap()
    }
}

impl MakeReply for TestServer {
    async fn make_reply<'a>(&self, topic_id: i64, post: &'a str) {
        #[derive(Serialize)]
        struct Form<'a> {
            post: &'a str,
        }
        let f = Form { post };

        self.post(format!("/posts/{}", topic_id).as_str())
            .form(&f)
            .await
            .assert_status_ok();
    }
}
