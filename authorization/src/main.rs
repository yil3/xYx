use anyhow::Context;
use axum::Router;
use x_core::application::Application;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Application::serve(Router::new())
        .run()
        .await
        .context("could not initialize application routes")?;
    Ok(())
}
