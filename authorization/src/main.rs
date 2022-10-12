use anyhow::Context;
use axum::{routing::get, Json, Router};
use x_core::application::{Application, TEXT};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("{:?}", TEXT.to_owned());
    Application::serve(Router::new().route("/", get(|| async { Json("hello world") })))
        .run()
        .await
        .context("could not initialize application routes")?;
    Ok(())
}
