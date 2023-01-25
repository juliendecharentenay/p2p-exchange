mod macros;
pub mod offer;
pub mod answer;
pub mod message;

#[cfg(feature = "lambda")]
pub mod lambda;

use rand::Rng;
pub fn gen_key() -> String {
  rand::thread_rng()
  .sample_iter(&rand::distributions::Alphanumeric)
  .take(4)
  .map(char::from)
  .collect::<String>()
  .to_uppercase()
}

use std::ops::Sub;
pub fn make_old_date() -> chrono::DateTime<chrono::Utc> {
    chrono::Utc::now().sub(chrono::naive::Days::new(1))
}

#[derive(derive_builder::Builder)]
pub struct AppState {
  filename: String,
}

impl Default for AppState {
  fn default() -> Self {
    AppState { filename: "./test.db3".to_string(), }
  }
}

impl AppState {
  pub fn db(&self) -> Result<rusqlite::Connection, Box<dyn std::error::Error>> {
    Ok(rusqlite::Connection::open(self.filename.as_str())?)
  }
}


