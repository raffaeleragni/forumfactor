use velvet_web::prelude::*;

#[tokio::main]
async fn main() {
    #[derive(RustEmbed)]
    #[folder = "static"]
    struct S;

    JWT::Secret.setup().await.unwrap();

    dotenv::dotenv().ok();
    let db = sqlite().await;
    sqlx::migrate!().run(&db).await.unwrap();

    App::new()
        .router(app())
        .inject(db)
        .statics::<S>()
        .start()
        .await;
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
