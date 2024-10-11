mod app;

#[tokio::main]
async fn main() {
    app::app().await.start().await.unwrap();
}
