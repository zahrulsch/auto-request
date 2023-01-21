#[cfg(test)]
mod test_shopee {
  use auto_requester::{Shopee, UrlType, ReqMethod};
  use reqwest::header::HeaderValue;
  use auto_requester::prelude::*;

  fn set_object() -> Shopee {
    Shopee::new(
      "https://mall.shopee.co.id/api/v4/search/search_items?by=relevancy&keyword=lampu&newest=0&order=desc&page=1&page_type=search&scenario=PAGE_GLOBAL_SEARCH&version=2", 
      UrlType::Search,
      ReqMethod::Get
    )
  }

  #[test]
  fn validate_url_isset() {
    let shopee = set_object();
    assert_eq!(shopee.url, "https://mall.shopee.co.id/api/v4/search/search_items?by=relevancy&keyword=lampu&newest=0&order=desc&page=1&page_type=search&scenario=PAGE_GLOBAL_SEARCH&version=2");
    assert_eq!(shopee.url, "https://mall.shopee.co.id/api/v4/search/search_items?by=relevancy&keyword=lampu&newest=0&order=desc&page=1&page_type=search&scenario=PAGE_GLOBAL_SEARCH&version=2");
  }

  #[test]
  fn validate_url_type() {
    let shopee = set_object();
    assert_eq!(shopee.url, "https://mall.shopee.co.id/api/v4/search/search_items?by=relevancy&keyword=lampu&newest=0&order=desc&page=1&page_type=search&scenario=PAGE_GLOBAL_SEARCH&version=2");
    assert_eq!(shopee.url_type, UrlType::Search);
  }

  #[test]
  fn validate_headers_is_setup() {
    let shopee = set_object();
    assert!(!shopee.headers.is_empty());
  }

  #[test]
  fn validate_add_ext_header() {
    let mut shopee = set_object();
    shopee.ext_headers("af-ac-enc-dat", "some_data");
    let af_enc_data = shopee.headers.get("af-ac-enc-dat");

    assert!(shopee.headers.contains_key("af-ac-enc-dat"));
    assert_eq!(af_enc_data, Some(&HeaderValue::from_static("some_data")));
  }
}