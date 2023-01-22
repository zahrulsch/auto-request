#[cfg(test)]
mod test_shopee_filter_config {
    use auto_requester::prelude::*;
    use auto_requester::{Shopee, UrlType, ReqMethod};

    #[tokio::test]
    async fn pick_config() -> Result<()> {
        let url = "https://mall.shopee.co.id/api/v4/search/search_filter_config?page_type=search";

        let mut client = Shopee::new(url, UrlType::Search, ReqMethod::Get);
        client.ext_headers("af-ac-enc-dat", "YWNzCjAwMgDugQOnbdg7w4UBAAABAQEAAAEAAFvQ8J+4XpFX3pXtqzjGsBeheidI9jgMJuBY5YwUnGgkbD8tbZKl7MD8wulNpdZ+Xkp7H3NnDj9EY+J9k9iN2mWqBzqRwnUaJ9Uo+mwN0BTEWKUFFyaGah8CQttjDdnx5aTGaILfJvkkAsofnY3Fbr8wAbIPZbDl2tc2tjRnGt4/NhNzJ3guD/Xs/wquoDF/AQ4HostDX42P962TeIRd3KHqf5T7KWlQLSZAUl/1tYL5oueF+MMrDa6SLPmgIy7rL+5VJEr2dtW1fviMiqVI6rhfz0hgJd/IjKuSZHu8hWsmkWmd/E/6LiUg0pgGmm3QwrTfvE7e8DuAmIr9CfSuuwg=");

         client.get_filter_config().await?;

        Ok(())
    }
}