use crate::{error::OhMyError, client::{UrlType, ReqMethod}};
use async_trait::async_trait;
use reqwest::{header::{HeaderMap, IntoHeaderName}, Response};

#[async_trait]
pub trait CommonRequester {
  fn new(url: impl Into<String>, url_type: UrlType, method: ReqMethod) -> Self;
  fn gen_headers() -> HeaderMap;
  fn ext_headers<T>(&mut self, key: T, value: &'static str) where T: IntoHeaderName;
  async fn send_request(&self) -> Result<Response>;
}

pub type Result<T> = core::result::Result<T, OhMyError>;