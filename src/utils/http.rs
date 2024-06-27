use std::time::Duration;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use serde::de::DeserializeOwned;

pub async fn get<T>(url:&str) -> Result<T, Box<dyn std::error::Error>> where T: DeserializeOwned{
    let mut headers = HeaderMap::new();
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static(
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Safari/537.36",
        ),
    );
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .headers(headers).timeout(Duration::new(10,0))
        .send()
        .await?
        .json::<T>()
        .await?;
    Ok(response)
}