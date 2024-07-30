use velvet_web::prelude::*;

#[tokio::main]
async fn main() {
    #[derive(RustEmbed)]
    #[folder = "static"]
    struct S;

    dotenv::dotenv().ok();

    App::new().router(app()).statics::<S>().start().await;
}

fn app() -> Router {
    Router::new().route("/", get(index))
}

#[derive(Template)]
#[template(path = "index.html")]
struct Index;

async fn index() -> Index {
    Index {}
}
