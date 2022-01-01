use super::api::routes;
use super::utils::logger;
use actix_cors::Cors;
use actix_web::{http::header, middleware, web, App, HttpServer, HttpResponse};
use dotenv::dotenv;
use middleware::Logger as ActixLogger;
use std::env;
use std::io::Result as IOResult;
use super::types::Result as ResultT;

fn enable_cors() -> Cors {
  let cors = Cors::default()
    .allow_any_origin()
    .allow_any_method()
    .allowed_headers(vec![
      header::AUTHORIZATION,
      header::ACCEPT,
      header::CONTENT_TYPE,
    ])
    .supports_credentials()
    .max_age(3600);

  cors
}

fn get_server_host() -> String {
  dotenv().ok();

  let default_host = "localhost".to_string();
  let server_host = env::var("SERVER_HOST").ok();

  let result = match server_host {
    Some(p) => {
      let empty_str = "".to_string();

      if p == empty_str {
        default_host
      } else {
        p
      }
    }
    None => default_host,
  };
  result
}

pub async fn actix() -> ResultT<IOResult<()>> {
  logger::init_logger(true);

  let listen_host = format!("{}:8080", get_server_host());
  let server = HttpServer::new(move || {
    App::new()
      .wrap(enable_cors())
      .wrap(ActixLogger::default())
      .service(routes::s3::index)
      .service(
        // # Scoped routes parent: /s3
        // # with query parameters
        // /s3/presigned?filekey=...
        web::scope("/s3/")
          .service(
            web::resource("/presigned")
              .route(web::get().to(routes::s3::get_presigned_url))
          )
          .service(
            web::resource("/objects")
              .route(web::get().to(routes::s3::get_list_objects))
          )
          .service(
            web::resource("/delete")
              .route(web::delete().to(routes::s3::delete_object))
          )
      )
      .default_service(
        web::route().to(|| HttpResponse::NotFound().json::<String>("Routes Not Found".into()))
      )
  })
  .client_timeout(500)
  .client_shutdown(500)
  .shutdown_timeout(1)
  .bind(listen_host)?
  .run()
  .await;

  Ok(server)
}
