mod lib;
use lib::{cli, s3::Result};

#[tokio::main]
async fn main() -> Result<()> {
    cli::run().await?;

    Ok(())
}
