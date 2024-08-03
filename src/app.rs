use velvet_web::prelude::*;

pub fn app() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/topics", get(topics))
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
struct Topics;

async fn topics() -> Topics {
    Topics {}
}

#[derive(Template)]
#[template(path = "posts.html")]
struct Posts;

async fn posts() -> Posts {
    Posts {}
}
