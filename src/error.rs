use thiserror::Error;
use reqwest::Error as RequestError;
use url::ParseError;
use serde_json::Error as JSONError;

#[derive(Error, Debug)]
pub enum OhMyError {
  #[error("client error")]
  Client (
    #[from]
    RequestError
  ),
  #[error("URL error")]
  URL (
    #[from]
    ParseError
  ),
  #[error("parse JSON error")]
  JSON (
    #[from]
    JSONError
  )
}