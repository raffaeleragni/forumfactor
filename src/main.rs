mod app;

use velvet_web::prelude::*;

#[tokio::main]
async fn main() {
    #[derive(RustEmbed)]
    #[folder = "static"]
    struct S;

    #[derive(Deserialize)]
    #[allow(dead_code)]
    struct Claims {
        username: String,
    }

    dotenvy::dotenv().ok();
    let db = sqlite().await;
    sqlx::migrate!().run(&db).await.unwrap();

    App::new()
        .router(app::app().authorized_cookie_claims("/login", |_: Claims| Ok(AuthResult::OK)))
        .login_flow(&db)
        .await
        .inject(db)
        .statics::<S>()
        .start()
        .await
        .unwrap();
}
