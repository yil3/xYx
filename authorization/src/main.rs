use authorization::router::router;
use x_core::application::Application;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Application::serve(router()).run().await?;
    Ok(())
}
