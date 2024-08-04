use velvet_web::prelude::*;

pub fn app() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/topics", get(topics).post(new_topic))
        .route("/posts", get(posts))
}

#[derive(Template)]
#[template(path = "index.html")]
struct Index;

async fn index() -> Index {
    Index {}
}

#[derive(Template)]
#[template(path = "topics.html")]
struct Topics {
    topics: Vec<Topic>,
}

#[derive(Debug)]
struct Topic {
    id: i64,
    title: String,
}

async fn topics(Extension(db): Extension<Pool<Sqlite>>) -> Topics {
    #[derive(Debug)]
    struct InnerTopic {
        id: i64,
        title: Option<String>,
    }
    let topics = query_as!(InnerTopic, "select id,title from topics")
        .fetch_all(&db)
        .await
        .unwrap()
        .into_iter()
        .map(|x| Topic {
            id: x.id,
            title: x.title.unwrap_or("".to_string()),
        })
        .collect();
    Topics { topics }
}

#[derive(Template)]
#[template(path = "posts.html")]
struct Posts;

async fn posts() -> Posts {
    Posts {}
}

#[derive(Deserialize)]
struct PostATopic {
    title: String,
}

async fn new_topic(Extension(db): Extension<Pool<Sqlite>>, Form(form): Form<PostATopic>) {
    query!(
        "insert into topics (id, title) values (null, ?)",
        form.title
    )
    .execute(&db)
    .await
    .unwrap();
}
