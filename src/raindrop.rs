use crate::highlights::Highlights;
use reqwest::{Client, Url};
use std::error::Error;

#[derive(Debug)]
pub struct RaindropClient {
    base_url: Url,
    access_token: String,
    client: Client,
}

impl RaindropClient {
    pub fn new(access_token: String) -> Result<Self, Box<dyn Error>> {
        Ok(RaindropClient {
            base_url: "https://api.raindrop.io".parse()?,
            client: Client::new(),
            access_token,
        })
    }

    pub async fn highlights(&self) -> Result<Highlights, Box<dyn Error>> {
        self.fetch_highlights(1).await
    }

    async fn fetch_highlights(&self, page: usize) -> Result<Highlights, Box<dyn Error>> {
        let mut results = Highlights::default();
        let mut page = page;

        loop {
            let per_page = 50;
            let endpoint = "/rest/v1/highlights";
            let url = self.base_url.join(endpoint)?;

            let response = self
                .client
                .get(url)
                .header("Authorization", format!("Bearer {}", self.access_token))
                .query(&vec![("perpage", per_page), ("page", page)])
                .send()
                .await?;

            let body: Highlights = response.json().await?;
            let items_fetched = body.items.len();
            results.merge(body);

            match items_fetched == per_page {
                true => page += 1,
                _ => return Ok(results),
            }
        }
    }
}
