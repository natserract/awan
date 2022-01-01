
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

// # Problems: `
//  thread 'actix:..' panicked at 'there is no reactor running, must be called from the context 
//  of a Tokio 1.x runtime'`
//
// So, we should handle this using `block_on()`
// This used for execute the future, blocking the current thread until completion
// 
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
