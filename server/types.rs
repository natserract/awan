
pub type Error = anyhow::Error;
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum ErrorE {
  BadRequest(String),
  CannotDecodeJwtToken(String),
  CannotEncodeJwtToken(String),
  InternalServerError(String),
  Unauthorized,
  Forbidden,
  NotFound(String),
  PoolError(String),
  ValidationError(Vec<String>),
  UnprocessableEntity(String),
  BlockingError(String),
}