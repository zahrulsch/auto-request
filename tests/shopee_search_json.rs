#[cfg(test)]
mod test_shopee_search_json {
  use auto_requester::{Shopee, UrlType};
  use auto_requester::prelude::*;

  #[tokio::test]
  async fn test_request() -> Result<()> {
    let mut shopee = Shopee::new(
      "https://mall.shopee.co.id/api/v4/search/search_items?by=relevancy&keyword=solder&newest=0&order=desc&page=2&page_type=search&scenario=PAGE_GLOBAL_SEARCH&version=2",
      UrlType::Search,
      auto_requester::ReqMethod::Get
    );

    shopee.ext_headers("af-ac-enc-dat", "YWNzCjAwMgDugQOnbdg7w4UBAAABAQEAAAEAAFvQ8J+4XpFX3pXtqzjGsBeheidI9jgMJuBY5YwUnGgkbD8tbZKl7MD8wulNpdZ+Xkp7H3NnDj9EY+J9k9iN2mWqBzqRwnUaJ9Uo+mwN0BTEWKUFFyaGah8CQttjDdnx5aTGaILfJvkkAsofnY3Fbr8wAbIPZbDl2tc2tjRnGt4/NhNzJ3guD/Xs/wquoDF/AQ4HostDX42P962TeIRd3KHqf5T7KWlQLSZAUl/1tYL5oueF+MMrDa6SLPmgIy7rL+5VJEr2dtW1fviMiqVI6rhfz0hgJd/IjKuSZHu8hWsmkWmd/E/6LiUg0pgGmm3QwrTfvE7e8DuAmIr9CfSuuwg=");

    let response_json = shopee.get_search_data().await?;

    response_json.items.iter()
      .for_each(|item| {
        println!("name: {}", item.item_basic.name);
        println!("-------------");
      });

    Ok(())
  }
}