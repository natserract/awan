use env_logger::builder as log_builder;
use std::env;

const RUST_LOG: &str = "RUST_LOG";

// Reference @see: https://github.com/Tapalogi/game-room/blob/main/src/utils.rs
pub fn init_logger(debug_mode: bool) {
  if env::var(RUST_LOG).is_err() {
      #[cfg(debug_assertions)]
      {
          if debug_mode {
              env::set_var(RUST_LOG, "trace");
          } else {
              env::set_var(RUST_LOG, "debug");
          }
      }
      #[cfg(not(debug_assertions))]
      {
          if debug_mode {
              env::set_var(RUST_LOG, "info");
          } else {
              env::set_var(RUST_LOG, "warn");
          }
      }
  }

  log_builder()
    .default_format()
    .format_timestamp_nanos()
    .format_indent(Some(4)).init();
}