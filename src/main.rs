use anyhow::Ok;
use axum::Server;
use tasque::app::create_app;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
  tracing_subscriber::fmt::init();

  Server::bind(&"0.0.0.0:3003".parse()?)
    .serve(create_app().into_make_service())
    .await?;
  Ok(())
}
