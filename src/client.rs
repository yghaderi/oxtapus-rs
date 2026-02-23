use crate::error::{Error, Result};
use crate::tsetmc::models::{History, MarketWatch, OptionMarketWatch};
use reqwest::{
    header::{HeaderMap, HeaderValue, USER_AGENT},
    Client,
};
use std::time::Duration;

#[derive(Clone)]
pub struct TsetmcClient {
    http: Client,
}

impl TsetmcClient {
    pub fn new() -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
             USER_AGENT,
             HeaderValue::from_static(
                 "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Safari/537.36",
             ),
         );
        let http = Client::builder()
            .user_agent("oxtapus-rs/0.2")
            .timeout(Duration::from_secs(2000))
            .gzip(true)
            .default_headers(headers)
            .build()
            .expect("Failed to build HTTP client");

        Self { http }
    }

    pub async fn mw(&self) -> Result<Vec<MarketWatch>> {
        const URL: &str = "https://cdn.tsetmc.com/api//ClosingPrice/GetMarketWatch?market=1&industrialGroup=&paperTypes%5B0%5D=8&showTraded=false&withBestLimits=true&hEven=0&RefID=0";

        let resp = self.http.get(URL).send().await?;

        if !resp.status().is_success() {
            return Err(Error::Api(format!("HTTP {}", resp.status())));
        }

        let data: Vec<MarketWatch> = resp.json().await?;
        Ok(data)
    }

    pub async fn option_market_watch(&self) -> Result<OptionMarketWatch> {
        const URL: &str = "https://cdn.tsetmc.com/api/Instrument/GetInstrumentOptionMarketWatch/1";

        let resp = self.http.get(URL).send().await?;

        if !resp.status().is_success() {
            return Err(Error::Api(format!("HTTP {}", resp.status())));
        }

        let data: OptionMarketWatch = resp.json::<OptionMarketWatch>().await?;

        Ok(data)
    }

    pub async fn history(&self, ins_code: &str) -> Result<Vec<History>> {
        let url = format!(
            "https://cdn.tsetmc.com/api/ClosingPrice/GetClosingPriceDailyList/{}/0",
            ins_code
        );

        let resp = self.http.get(url).send().await?;

        if !resp.status().is_success() {
            return Err(Error::Api(format!("HTTP {}", resp.status())));
        }

        let data: Vec<History> = resp.json().await?;
        Ok(data)
    }
}

impl Default for TsetmcClient {
    fn default() -> Self {
        Self::new()
    }
}
