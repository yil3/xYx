use anyhow::Context;
use user_resources::controller::init_router;
use x_core::application::Application;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Application::serve(init_router())
        .run()
        .await
        .context("could not initialize application routes")?;
    Ok(())
}
