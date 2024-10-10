mod app;

use app::Claims;
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
        .router(app::app().authorized_cookie_claims("/login", |_: Claims| Ok(AuthResult::OK)))
        .login_flow_with_mail(&db)
        .await
        .inject(db)
        .inject(mailer())
        .statics::<S>()
        .start()
        .await
        .unwrap();
}
