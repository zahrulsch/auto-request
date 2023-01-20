use thiserror::Error;
use reqwest::Error as RequestError;
use url::ParseError;

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
  )
}