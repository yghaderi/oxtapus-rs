mod models;
mod utils;

pub mod tsetmc {
    use crate::models::tsetmc::OptionMarketWatch;
    use crate::utils::http::get;
    use std::error::Error;

    fn url(endpoint:&str)->String{
        const  BASE_URL: &str ="https://cdn.tsetmc.com/api";
        format!("{BASE_URL}{endpoint}")
    }
    pub async  fn tse_option_mw()->Result<OptionMarketWatch, Box<dyn Error>> {
        let endpoint = "/Instrument/GetInstrumentOptionMarketWatch/1";
        let r = get::<OptionMarketWatch>(&url(endpoint)).await?;
        Ok(r)
    }
}


