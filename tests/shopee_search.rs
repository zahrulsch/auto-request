#[cfg(test)]
mod test_shopee_search {
  use auto_requester::{Shopee, UrlType, ReqMethod};
  use auto_requester::prelude::*;
  use serde_json::Value;

  #[tokio::test]
  async fn test_send_request() -> Result<()> {

    let by = ["relevancy", "ctime", "sales"];

    for b in by.iter() {
      let url = String::from("https://mall.shopee.co.id/api/v4/search/search_items?by={by}&keyword=minyak&newest=0&order=desc&page=1&page_type=search&scenario=PAGE_GLOBAL_SEARCH&version=2")
        .replace("{by}", b);

      let mut shopee = Shopee::new(
        url, 
        UrlType::Search,
        ReqMethod::Get
      );
      let additional = "YWNzCjAwMgDugQOnbdg7w4UBAAABAQEAAAEAAFvQ8J+4XpFX3pXtqzjGsBeheidI9jgMJuBY5YwUnGgkbD8tbZKl7MD8wulNpdZ+Xkp7H3NnDj9EY+J9k9iN2mWqBzqRwnUaJ9Uo+mwN0BTEWKUFFyaGah8CQttjDdnx5aTGaILfJvkkAsofnY3Fbr8wAbIPZbDl2tc2tjRnGt4/NhNzJ3guD/Xs/wquoDF/AQ4HostDX42P962TeIRd3KHqf5T7KWlQLSZAUl/1tYL5oueF+MMrDa6SLPmgIy7rL+5VJEr2dtW1fviMiqVI6rhfz0hgJd/IjKuSZHu8hWsmkWmd/E/6LiUg0pgGmm3QwrTfvE7e8DuAmIr9CfSuuwg=";
  
      shopee.ext_headers("af-ac-enc-dat", additional);
      
      let response = shopee.send_request().await?;
      if let Ok(response) = response.json::<Value>().await {
        if let Some(items) = response.get("items") {
          assert!(items.is_array());
  
          match items {
            Value::Null => {},
            Value::Bool(_) => {},
            Value::Number(_) => {},
            Value::String(_) => {},
            Value::Array(items) => {
              let names = items.iter()
                .map(|i| {
                  let item_basic = i.as_object().unwrap().get("item_basic");
                  let name = item_basic.unwrap().as_object().unwrap().get("name").unwrap();
                  // let name = item_basic.get("name").unwrap();
                  name.as_str().unwrap()
                })
                .collect::<Vec<&str>>();

              println!("{:?}", names);
              assert!(items.len() > 10)
            },
            Value::Object(_) => {},
          }
        };
      };
    }

    Ok(())
  }

}