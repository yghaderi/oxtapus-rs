pub mod models;
mod utils;

/// Get data from tsetmc.com
pub mod tsetmc {
    use crate::models::tsetmc::{History, MarketWatch, OptionMarketWatch};
    use crate::utils::http::get;
    use std::error::Error;

    fn url(endpoint: &str) -> String {
        const BASE_URL: &str = "https://cdn.tsetmc.com/api";
        format!("{BASE_URL}{endpoint}")
    }

    /// Get market-watch data.
    /// # Examples
    /// ```
    /// use oxtapus::tsetmc;
    /// use std::error::Error;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn Error>> {
    ///    let data = tsetmc::mw().await?;
    ///    println!("{:#?}", data[0].records[0]);
    ///    Ok(())
    ///}
    /// ```
    pub async fn mw() -> Result<Vec<MarketWatch>, Box<dyn Error>> {
        let endpoint = "/ClosingPrice/GetMarketWatch?market=1&industrialGroup=&paperTypes%5B0%5D=8&showTraded=false&withBestLimits=true&hEven=0&RefID=0";
        let urls = vec![url(endpoint)];
        let r = get::<MarketWatch>(&urls).await?;
        Ok(r)
    }

    /// Get options data.
    /// # Examples
    /// ```
    /// use oxtapus::tsetmc;
    /// use std::error::Error;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn Error>> {
    ///    let data = tsetmc::options_mw().await?;
    ///    println!("{:#?}", data[0].records[0]);
    ///    Ok(())
    ///}
    /// ```
    pub async fn options_mw() -> Result<Vec<OptionMarketWatch>, Box<dyn Error>> {
        let endpoint = "/Instrument/GetInstrumentOptionMarketWatch/0";
        let urls = vec![url(endpoint)];
        let r = get::<OptionMarketWatch>(&urls).await?;
        Ok(r)
    }

    /// Get trades-hist data.
    /// # Examples
    /// ```
    /// use oxtapus::tsetmc;
    /// use std::error::Error;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn Error>> {
    ///    let data = tsetmc::history(vec![String::from("46348559193224090")]).await?;
    ///    println!("{:#?}", data[0].records[0]);
    ///    Ok(())
    ///}
    /// ```
    pub async fn history(ins_codes: Vec<String>) -> Result<Vec<History>, Box<dyn Error>> {
        let urls: Vec<String> = ins_codes
            .into_iter()
            .map(|i| url(&format!("/ClosingPrice/GetClosingPriceDailyList/{i}/0")))
            .collect();
        let r = get::<History>(&urls).await?;
        Ok(r)
    }
}
