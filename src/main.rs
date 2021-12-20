use std::env;
use dotenv::dotenv;

fn main() {
    dotenv().ok();

    let database_url = env::var("APP_BUCKET").expect("env must be set!");
    println!("Hello, world! {}", database_url);
}
