use thiserror::Error;
use reqwest::Error as RequestError;
use url::ParseError;
use serde_json::Error as JSONError;

#[derive(Error, Debug)]
pub enum OhMyError {
  #[error(transparent)]
  Client (
    #[from]
    RequestError
  ),
  #[error(transparent)]
  URL (
    #[from]
    ParseError
  ),
  #[error(transparent)]
  JSON (
    #[from]
    JSONError
  )
}
