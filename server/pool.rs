use actix_web::{App, main as actix_main, HttpServer, middleware, http::header};
use actix_cors::Cors;
use middleware::Logger as ActixLogger;
use dotenv::dotenv;
use std::env;
use std::io::Result as IOResult;
use crate::api::routes;
use crate::utils::logger;

fn enable_cors() -> Cors {
  let cors = 
    Cors::default()
    .allow_any_origin()
    .allowed_methods(vec!["GET", "POST"])
    .allowed_headers(vec!(header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE))
    .supports_credentials()
    .max_age(3600);

  cors
}

fn get_server_host() -> String {
  dotenv().ok();

  let default_host = "localhost".to_string();
  let server_host = env::var("SERVER_HOST").ok();
  
  let result = 
    match server_host {
      Some (p) => {
        let empty_str = "".to_string();

        if p == empty_str { 
          default_host 
        } else {
          p
        }
      },
      None => default_host
    };

  result
}

#[actix_main]
pub async fn actix() -> IOResult<()> {
    logger::init_logger(true);

    let server_host = get_server_host();
    let listen_host = format!("{}:8000", server_host);

    let server = HttpServer::new(move || {
      App::new()
        .wrap(enable_cors())
        .wrap(ActixLogger::default())
        .service(
          routes::s3_objects::index
        )
    })
    .client_timeout(500)
    .client_shutdown(500)
    .shutdown_timeout(1)
    .bind(listen_host)?
    .run()
    .await;
    
    server
}
