use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
struct OptionMarketWatchRow {
    #[serde(alias = "insCode_P")]
    ins_code_p: String,
    #[serde(alias = "insCode_C")]
    ins_code_c: String,
    #[serde(alias = "contractSize")]
    contract_size: f64,
    #[serde(alias = "uaInsCode")]
    ins_code_ua: String,
    #[serde(alias = "lVal18AFC_P")]
    symbol_p: String,
    #[serde(alias = "lVal30_P")]
    name_p: String,
    #[serde(alias = "zTotTran_P")]
    trade_count_p: f64,
    #[serde(alias = "qTotTran5J_P")]
    traded_vol_p: f64,
    #[serde(alias = "qTotCap_P")]
    traded_val_p: f64,
    #[serde(alias = "notionalValue_P")]
    notional_value_p: f64,
    #[serde(alias = "pClosing_P")]
    final_p: f64,
    #[serde(alias = "priceYesterday_P")]
    y_final_p: f64,
    #[serde(alias = "oP_P")]
    open_interest_p: f64,
    #[serde(alias = "pDrCotVal_P")]
    close_p: f64,
    #[serde(alias = "lval30_UA")]
    symbol_ua: String,
    #[serde(alias = "pClosing_UA")]
    final_ua: f64,
    #[serde(alias = "priceYesterday_UA")]
    y_final_ua: f64,
    #[serde(alias = "beginDate")]
    begin_date: String,
    #[serde(alias = "endDate")]
    end_date: String,
    #[serde(alias = "strikePrice")]
    strike_price: f64,
    #[serde(alias = "remainedDay")]
    t: f64,
    #[serde(alias = "pDrCotVal_C")]
    close_c: f64,
    #[serde(alias = "oP_C")]
    open_interest_c: f64,
    #[serde(alias = "pClosing_C")]
    final_c: f64,
    #[serde(alias = "priceYesterday_C")]
    y_final_c: f64,
    #[serde(alias = "notionalValue_C")]
    notional_value_c: f64,
    #[serde(alias = "qTotCap_C")]
    traded_val_c: f64,
    #[serde(alias = "qTotTran5J_C")]
    traded_vol_c: f64,
    #[serde(alias = "zTotTran_C")]
    trade_count: f64,
    #[serde(alias = "lVal30_C")]
    name_c: String,
    #[serde(alias = "lVal18AFC_C")]
    symbol_c: String,
    #[serde(alias = "pMeDem_P")]
    bid_price_p: f64,
    #[serde(alias = "qTitMeDem_P")]
    bid_vol_p: f64,
    #[serde(alias = "pMeOf_P")]
    ask_price_p: f64,
    #[serde(alias = "qTitMeOf_P")]
    ask_vol_p: f64,
    #[serde(alias = "pMeDem_C")]
    bid_price_c: f64,
    #[serde(alias = "qTitMeDem_C")]
    bid_vol_c: f64,
    #[serde(alias = "pMeOf_C")]
    ask_price_c: f64,
    #[serde(alias = "qTitMeOf_C")]
    ask_vol_c: f64,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct OptionMarketWatch {
    #[serde(alias = "instrumentOptMarketWatch")]
    records: Vec<OptionMarketWatchRow>,
}

#[derive(Serialize, Deserialize, Debug)]
struct OrderBook{
    #[serde(alias = "n")]
    quote: f64,
    #[serde(alias = "zmd")]
    bid_count: f64,
    #[serde(alias = "qmd")]
    bid_vol: f64,
    #[serde(alias = "pmd")]
    bid_price: f64,
    #[serde(alias = "pmo")]
    ask_price: f64,
    #[serde(alias = "qmo")]
    ask_vol: f64,
    #[serde(alias = "zmo")]
    ask_count: f64,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MarketWatchRow{
    #[serde(alias = "blDs")]
    order_book: Vec<OrderBook>,
    eps: f64,
    #[serde(alias = "hEven")]
    hour_event: f64,
    #[serde(alias = "insCode")]
    ins_code: String,
    #[serde(alias = "insID")]
    ins_id: String,
    #[serde(alias = "lva")]
    symbol:String,
    #[serde(alias = "lvc")]
    name:String,
    #[serde(alias = "pMax")]
    max_price: f64,
    #[serde(alias = "pMin")]
    min_price: f64,
    #[serde(alias = "pcl")]
    final_:f64,
    #[serde(alias = "pdv")]
    close:f64,
    pe: Option<String>,
    #[serde(alias = "pf")]
    open: f64,
    #[serde(alias = "pmn")]
    low:f64,
    #[serde(alias = "pmx")]
    high: f64,
    #[serde(alias = "py")]
    y_final:f64,
    #[serde(alias = "qtc")]
    traded_val: f64,
    #[serde(alias = "qtj")]
    traded_vol: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MarketWatch{
    #[serde(alias = "marketwatch")]
    records: Vec<MarketWatchRow>
}