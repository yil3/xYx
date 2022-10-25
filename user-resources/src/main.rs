use anyhow::Context;
use user_resources::router::router;
use x_core::application::Application;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Application::serve(router())
        .run()
        .await
        .context("could not initialize application routes")?;
    Ok(())
}
