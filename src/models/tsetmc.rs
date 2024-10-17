use chrono::NaiveDate;
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OptionMarketWatchRow {
    #[serde(alias = "insCode_P")]
    pub ins_code_p: String,
    #[serde(alias = "insCode_C")]
    pub ins_code_c: String,
    #[serde(alias = "contractSize")]
    pub contract_size: f64,
    #[serde(alias = "uaInsCode")]
    pub ins_code_ua: String,
    #[serde(alias = "lVal18AFC_P")]
    pub symbol_p: String,
    #[serde(alias = "lVal30_P")]
    pub name_p: String,
    #[serde(alias = "zTotTran_P")]
    pub trade_count_p: f64,
    #[serde(alias = "qTotTran5J_P")]
    pub traded_vol_p: f64,
    #[serde(alias = "qTotCap_P")]
    pub traded_val_p: f64,
    #[serde(alias = "notionalValue_P")]
    pub notional_value_p: f64,
    #[serde(alias = "pClosing_P")]
    pub final_p: f64,
    #[serde(alias = "priceYesterday_P")]
    pub y_final_p: f64,
    #[serde(alias = "oP_P")]
    pub open_interest_p: f64,
    #[serde(alias = "pDrCotVal_P")]
    pub close_p: f64,
    #[serde(alias = "lval30_UA")]
    pub symbol_ua: String,
    #[serde(alias = "pClosing_UA")]
    pub final_ua: f64,
    #[serde(alias = "priceYesterday_UA")]
    pub y_final_ua: f64,
    #[serde(alias = "beginDate")]
    pub begin_date: String,
    #[serde(alias = "endDate")]
    pub end_date: String,
    #[serde(alias = "strikePrice")]
    pub strike_price: f64,
    #[serde(alias = "remainedDay")]
    pub t: f64,
    #[serde(alias = "pDrCotVal_C")]
    pub close_c: f64,
    #[serde(alias = "oP_C")]
    pub open_interest_c: f64,
    #[serde(alias = "pClosing_C")]
    pub final_c: f64,
    #[serde(alias = "priceYesterday_C")]
    pub y_final_c: f64,
    #[serde(alias = "notionalValue_C")]
    pub notional_value_c: f64,
    #[serde(alias = "qTotCap_C")]
    pub traded_val_c: f64,
    #[serde(alias = "qTotTran5J_C")]
    pub traded_vol_c: f64,
    #[serde(alias = "zTotTran_C")]
    pub trade_count: f64,
    #[serde(alias = "lVal30_C")]
    pub name_c: String,
    #[serde(alias = "lVal18AFC_C")]
    pub symbol_c: String,
    #[serde(alias = "pMeDem_P")]
    pub bid_price_p: f64,
    #[serde(alias = "qTitMeDem_P")]
    pub bid_vol_p: f64,
    #[serde(alias = "pMeOf_P")]
    pub ask_price_p: f64,
    #[serde(alias = "qTitMeOf_P")]
    pub ask_vol_p: f64,
    #[serde(alias = "pMeDem_C")]
    pub bid_price_c: f64,
    #[serde(alias = "qTitMeDem_C")]
    pub bid_vol_c: f64,
    #[serde(alias = "pMeOf_C")]
    pub ask_price_c: f64,
    #[serde(alias = "qTitMeOf_C")]
    pub ask_vol_c: f64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OptionMarketWatch {
    #[serde(alias = "instrumentOptMarketWatch")]
    pub records: Vec<OptionMarketWatchRow>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct OrderBook {
    #[serde(alias = "n")]
    pub quote: f64,
    #[serde(alias = "zmd")]
    pub bid_count: f64,
    #[serde(alias = "qmd")]
    pub bid_vol: f64,
    #[serde(alias = "pmd")]
    pub bid_price: f64,
    #[serde(alias = "pmo")]
    pub ask_price: f64,
    #[serde(alias = "qmo")]
    pub ask_vol: f64,
    #[serde(alias = "zmo")]
    pub ask_count: f64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MarketWatchRow {
    #[serde(alias = "blDs")]
    pub order_book: Vec<OrderBook>,
    pub eps: f64,
    #[serde(alias = "hEven")]
    pub hour_event: f64,
    #[serde(alias = "insCode")]
    pub ins_code: String,
    #[serde(alias = "insID")]
    pub ins_id: String,
    #[serde(alias = "lva")]
    pub symbol: String,
    #[serde(alias = "lvc")]
    pub name: String,
    #[serde(alias = "pMax")]
    pub max_price: f64,
    #[serde(alias = "pMin")]
    pub min_price: f64,
    #[serde(alias = "pcl")]
    pub final_: f64,
    #[serde(alias = "pdv")]
    pub close: f64,
    pub pe: Option<String>,
    #[serde(alias = "pf")]
    pub open: f64,
    #[serde(alias = "pmn")]
    pub low: f64,
    #[serde(alias = "pmx")]
    pub high: f64,
    #[serde(alias = "py")]
    pub y_final: f64,
    #[serde(alias = "qtc")]
    pub traded_val: f64,
    #[serde(alias = "qtj")]
    pub traded_vol: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MarketWatch {
    #[serde(alias = "marketwatch")]
    pub records: Vec<MarketWatchRow>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HistoryRecord {
    #[serde(alias = "dEven", deserialize_with = "deserialize_date")]
    pub date: NaiveDate,
    #[serde(alias = "insCode")]
    pub ins_code: String,
    #[serde(alias = "priceFirst")]
    pub open: f64,
    #[serde(alias = "priceMax")]
    pub high: f64,
    #[serde(alias = "priceMin")]
    pub low: f64,
    #[serde(alias = "pDrCotVal")]
    pub close: f64,
    #[serde(alias = "pClosing")]
    pub final_: f64,
    #[serde(alias = "priceYesterday")]
    pub y_final: f64,
    #[serde(alias = "qTotTran5J")]
    pub volume: f64,
    #[serde(alias = "qTotCap")]
    pub value: f64,
    #[serde(alias = "zTotTran")]
    pub trade_count: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct History {
    #[serde(alias = "closingPriceDaily")]
    pub records: Vec<HistoryRecord>,
}

fn deserialize_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    struct DateVisitor;

    impl<'de> Visitor<'de> for DateVisitor {
        type Value = NaiveDate;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an integer or string representing a date in the format YYYYMMDD")
        }

        fn visit_u64<E>(self, value: u64) -> Result<NaiveDate, E>
        where
            E: de::Error,
        {
            let date_str = format!("{:08}", value);
            NaiveDate::parse_from_str(&date_str, "%Y%m%d")
                .map_err(|_| E::custom(format!("invalid date format: {}", date_str)))
        }
    }

    deserializer.deserialize_any(DateVisitor)
}
