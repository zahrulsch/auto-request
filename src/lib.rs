pub mod prelude;
pub mod error;

use reqwest::{header::{HeaderMap, HeaderValue, IntoHeaderName}, ClientBuilder, Url, Response,};
use async_trait::async_trait;
use prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum UrlType {
  Product,
  Search,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ReqMethod {
  Get,
  Post
}

#[async_trait]
pub trait CommonRequester {
  fn new(url: impl Into<String>, url_type: UrlType, method: ReqMethod) -> Self;
  fn gen_headers() -> HeaderMap;
  fn ext_headers<T>(&mut self, key: T, value: &'static str) where T: IntoHeaderName;
  async fn send_request(&self) -> Result<Response>;
}

#[derive(Debug)]
pub struct Shopee {
  pub url: String,
  pub url_type: UrlType,
  pub headers: HeaderMap,
  pub method: ReqMethod
}

#[async_trait]
impl CommonRequester for Shopee {
  fn gen_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();

    headers.append("accept-encoding", HeaderValue::from_static("gzip"));
    headers.append("connection", HeaderValue::from_static("Keep-Alive"));
    headers.append("shopee_http_dns_mode", HeaderValue::from_static("1"));
    headers.append("user-agent", HeaderValue::from_static("Android app Shopee appver=28308 app_type=1"));
    headers.append("x-api-source", HeaderValue::from_static("rn"));
    headers.append("x-shopee-language", HeaderValue::from_static("id"));

    headers
  }

  fn new(url: impl Into<String>, url_type: UrlType, method: ReqMethod) -> Self {
    let headers = Self::gen_headers();

    Self {
      url: url.into(),
      url_type,
      headers,
      method
    }
  }

  fn ext_headers<T>(&mut self, key: T, value: &'static str)
    where 
      T: IntoHeaderName,
  {
    self.headers.append(key, HeaderValue::from_static(value));
  }

  /** Don't forget to send additional headers like:
   * af-ac-enc-dat
   */
  async fn send_request(&self) -> Result<Response> {
    let builder = ClientBuilder::new();
    let client = builder
      .default_headers(self.headers.clone())
      .build()?;

    match self.method {
        ReqMethod::Post => {
          let url = Url::parse(self.url.as_str())?;

          let response = client
            .post(url)
            .send()
            .await?;

          Ok(response)
        },
        ReqMethod::Get => {
          let url = Url::parse(self.url.as_str())?;

          let response = client
            .get(url)
            .send()
            .await?;
            
          Ok(response)
        }
    }
  }
}


