mod utils;
mod pool;
mod api;
mod types;

use types::Result as ResultT;

#[tokio::main]
async fn main() -> ResultT<()> {
    pool::actix();

    Ok(())
}
