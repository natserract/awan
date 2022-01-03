use super::api::routes;
use super::utils::logger;
use super::types::Result as ResultT;
use actix_cors::Cors;
use actix_web::{http::header, middleware, web, App, HttpServer, HttpResponse};
use dotenv::dotenv;
use middleware::Logger as ActixLogger;
use std::env;
use std::io::Result as IOResult;
use crate::utils::generic::{OptChoice, Of};

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

fn get_server_env(var_name: &str, default_value: String) -> String {
  dotenv().ok();

  let env = env::var(var_name).ok();
  let init_env_val = OptChoice {
    default_value,
  };

  init_env_val.value(env)
}

pub async fn actix() -> ResultT<IOResult<()>> {
  logger::init_logger(true);

  let server_host = get_server_env("SERVER_HOST", "localhost".to_string());
  let server_port = get_server_env("SERVER_PORT", "8080".to_string());

  let listen_host = format!("{}:{}", server_host, server_port);
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
          .service(
            web::resource("/bucket")
              .route(web::get().to(routes::s3::get_bucket_name))
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
