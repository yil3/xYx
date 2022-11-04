use authorization::router::router;
use x_core::application::Application;

use time::format_description::well_known::Rfc3339;

pub fn test() {
    let a = datetime!(2019-01-01).format(&Rfc3339);
    println!("{}", a);
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    test();
    Application::serve(router()).run().await?;
    Ok(())
}
