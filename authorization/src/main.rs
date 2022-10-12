use anyhow::Context;
use axum::{routing::get, Json, Router};
use x_core::application::Application;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = Application::create_pool();
    println!("{pool:#?}");
    Application::serve(Router::new().route("/", get(|| async { Json("hello world") })))
        .run()
        .await
        .context("could not initialize application routes")?;
    Ok(())
}
