#![deny(clippy::all)]

use napi_derive::napi;

#[napi]
pub fn hello() -> String {
  "Hello from Rust!".to_string()
}
