mod app;

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
        .enable_compression()
        .router(app::app())
        .inject(db)
        .statics::<S>()
        .start()
        .await;
}
