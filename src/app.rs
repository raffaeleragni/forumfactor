use askama_axum::IntoResponse;
use velvet_web::prelude::*;

pub fn app() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/topics", get(topics).post(new_topic))
        .route("/posts/:topic_id", get(posts))
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
struct Posts {
    posts: Vec<Post>,
}

#[derive(Debug)]
struct Post {
    id: i64,
    post: String,
}

async fn posts(Extension(db): Extension<Pool<Sqlite>>, Path(topic_id): Path<i64>) -> Posts {
    #[derive(Debug)]
    struct InnerPost {
        id: i64,
        post: Option<String>,
    }
    let posts = query_as!(
        InnerPost,
        "select id,post from posts where topic_id=?",
        topic_id
    )
    .fetch_all(&db)
    .await
    .unwrap()
    .into_iter()
    .map(|x| Post {
        id: x.id,
        post: x.post.unwrap_or("".to_string()),
    })
    .collect();
    Posts { posts }
}

#[derive(Deserialize)]
struct PostATopic {
    title: String,
    post: String,
}

async fn new_topic(
    Extension(db): Extension<Pool<Sqlite>>,
    Form(form): Form<PostATopic>,
) -> impl IntoResponse {
    let topic_id = query!(
        "insert into topics (id, title) values (null, ?)",
        form.title
    )
    .execute(&db)
    .await
    .unwrap()
    .last_insert_rowid();

    query!(
        "insert into posts (id, topic_id, post) values (null, ?, ?)",
        topic_id,
        form.post
    )
    .execute(&db)
    .await
    .unwrap();

    let mut headers = HeaderMap::new();
    headers.insert(
        "ID",
        HeaderValue::from_str(format!("{topic_id}").as_str()).unwrap(),
    );
    (headers, "")
}
