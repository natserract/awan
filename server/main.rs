mod utils;
mod pool;
mod api;
mod types;

use types::Result as ResultT;
use pool::actix;

#[tokio::main]
async fn main() -> ResultT<()> {
    actix()?;

    Ok(())
}
