
#[macro_use] 
pub extern crate serde;
pub extern crate serde_json;

mod utils;
mod pool;
mod api;
mod types;
mod handlers;
mod cli;
mod middleware;

use actix_web::main as actix_main;
use types::Result as ResultT;
use pool::actix;

use tokio::runtime::Runtime;

pub fn runtime() -> Runtime {
  Runtime::new().unwrap()
}

// Actix-web internally uses Tokio
// You will be able to use all the usual Tokio utilities
#[actix_main]
async fn main() -> ResultT<()> {
    // cli::run().await?;
    actix().await?;

    Ok(())
}
