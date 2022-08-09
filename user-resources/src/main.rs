use user_resources::application::Application;

#[tokio::main]
async fn main() {
    Application::run().await.unwrap();
}
