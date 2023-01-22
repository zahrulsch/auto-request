use async_trait::async_trait;
use reqwest::{header::{HeaderMap, HeaderValue, IntoHeaderName}, Response, ClientBuilder, Url};
use crate::{prelude::*, model_translations::{ShopeeSearchResult, ShopeeFilterConfig}};

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

#[derive(Debug)]
pub struct Shopee {
  pub url: String,
  pub url_type: UrlType,
  pub headers: HeaderMap,
  pub method: ReqMethod
}

impl Shopee {
  pub async fn get_search_data(&self) -> Result<ShopeeSearchResult> {
    let builder = ClientBuilder::new();
    let client = builder
      .default_headers(self.headers.clone())
      .build()?;

    let url = Url::parse(self.url.as_str())?;

    let response = client
      .get(url)
      .send()
      .await?;

    let json_response = response.json::<ShopeeSearchResult>().await?;
    Ok(json_response)
  }

  pub async fn get_filter_config(&self) -> Result<ShopeeFilterConfig> {
    let builder = ClientBuilder::new();
    let client = builder
      .default_headers(self.headers.clone())
      .build()?;

    let url = Url::parse(self.url.as_str())?;

    let response = client
      .get(url)
      .send()
      .await?;

    let response = response.text().await?;
    let json_response = serde_json::from_str::<ShopeeFilterConfig>(&response)?;

    Ok(json_response)
  }
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

  /** 
   #### Provide DIRECT Android API like for 1st arg:
   - https://mall.shopee.co.id/api/v4/item/get?itemid=3293687772&shopid=379494847 (Single)
   - https://mall.shopee.co.id/api/v4/search/search_items?by=relevancy&keyword=lampu%20tidur&newest=0&order=desc&page=1&page_type=search&scenario=PAGE_GLOBAL_SEARCH&version=2 (Search)

   This method set default header before send request, but still need additional headers
   */
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
    > Example
   ```ignore
        let shopee = Shopee::new();
        shopee.ext_header("af-ac-enc-dat", "some_data");
        let response = shopee.send_request().await?;
   ```
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