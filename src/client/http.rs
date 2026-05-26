use reqwest::Client;
use reqwest::Error;

pub async fn fetch_url(url: &str) -> Result<reqwest::Response, Error> {
    let client = Client::new();

    let response = client.get(url).send().await?;

    Ok(response)
}
