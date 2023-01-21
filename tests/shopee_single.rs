#[cfg(test)]
mod test_shopee_single {
  use auto_requester::{Shopee, UrlType, ReqMethod};
  use auto_requester::prelude::*;
  use serde_json::Value;

  #[tokio::test]
  async fn test_pick_one_product() -> Result<()> {
    let mut shopee = Shopee::new(
      "https://mall.shopee.co.id/api/v4/item/get?itemid=3293687772&shopid=379494847",
      UrlType::Product,
      ReqMethod::Get
    );

    shopee.ext_headers("af-ac-enc-dat", "YWNzCjAwMgDugQOnbdg7w4UBAAABAQEAAAEAAFvQ8J+4XpFX3pXtqzjGsBeheidI9jgMJuBY5YwUnGgkbD8tbZKl7MD8wulNpdZ+Xkp7H3NnDj9EY+J9k9iN2mWqBzqRwnUaJ9Uo+mwN0BTEWKUFFyaGah8CQttjDdnx5aTGaILfJvkkAsofnY3Fbr8wAbIPZbDl2tc2tjRnGt4/NhNzJ3guD/Xs/wquoDF/AQ4HostDX42P962TeIRd3KHqf5T7KWlQLSZAUl/1tYL5oueF+MMrDa6SLPmgIy7rL+5VJEr2dtW1fviMiqVI6rhfz0hgJd/IjKuSZHu8hWsmkWmd/E/6LiUg0pgGmm3QwrTfvE7e8DuAmIr9CfSuuwg=");
    let response = shopee.send_request().await?;

    println!("{:#?}", response.json::<Value>().await.unwrap());

    Ok(())
  }
}