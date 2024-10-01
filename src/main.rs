mod app;

use velvet_web::prelude::*;

#[tokio::main]
async fn main() {
    #[derive(RustEmbed)]
    #[folder = "static"]
    struct S;

    dotenvy::dotenv().ok();
    let db = sqlite().await;
    sqlx::migrate!().run(&db).await.unwrap();

    App::new()
        .router(app::app())
        .inject(db)
        .statics::<S>()
        .start()
        .await
        .unwrap();
}
