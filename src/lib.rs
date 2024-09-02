mod models;
mod utils;

pub mod tsetmc {
    use crate::models::tsetmc::{History, MarketWatch, OptionMarketWatch};
    use crate::utils::http::get;
    use std::error::Error;

    fn url(endpoint: &str) -> String {
        const BASE_URL: &str = "https://cdn.tsetmc.com/api";
        format!("{BASE_URL}{endpoint}")
    }
    pub async fn mw() -> Result<MarketWatch, Box<dyn Error>> {
        let endpoint = "/ClosingPrice/GetMarketWatch?market=1&industrialGroup=&paperTypes%5B0%5D=8&showTraded=false&withBestLimits=true&hEven=0&RefID=0";
        let r = get::<MarketWatch>(&url(endpoint)).await?;
        Ok(r)
    }
    pub async fn tse_option_mw() -> Result<OptionMarketWatch, Box<dyn Error>> {
        let endpoint = "/Instrument/GetInstrumentOptionMarketWatch/1";
        let r = get::<OptionMarketWatch>(&url(endpoint)).await?;
        Ok(r)
    }
    pub async fn history(ins_code: String) -> Result<History, Box<dyn Error>> {
        let endpoint = format!("/ClosingPrice/GetClosingPriceDailyList/{ins_code}/0");
        let r = get::<History>(&url(&endpoint)).await?;
        Ok(r)
    }
}
