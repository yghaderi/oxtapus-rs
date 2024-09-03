use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use serde::de::DeserializeOwned;
use std::time::Duration;

pub async fn get<T>(url: &Vec<String>) -> Result<Vec<T>, Box<dyn std::error::Error>>
where
    T: DeserializeOwned,
{
    let mut headers = HeaderMap::new();
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static(
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Safari/537.36",
        ),
    );
    let client = reqwest::Client::builder()
        .use_rustls_tls()
        .http1_only()
        .default_headers(headers)
        .build()?;
    let mut response: Vec<T> = vec![];
    for i in url {
        let response_ = client
            .get(i)
            .timeout(Duration::new(10, 0))
            .send()
            .await?
            .json::<T>()
            .await?;
        response.push(response_);
    }
    Ok(response)
}
