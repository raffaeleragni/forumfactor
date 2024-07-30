use velvet_web::prelude::*;

pub fn app() -> Router {
    Router::new().route("/", get(index))
}

#[derive(Template)]
#[template(path = "index.html")]
struct Index;

async fn index() -> Index {
    Index {}
}
