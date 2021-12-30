mod lib;
use lib::{cli, types::Result as ResultT};

#[tokio::main]
async fn main() -> ResultT<()> {
    cli::run().await?;

    Ok(())
}
