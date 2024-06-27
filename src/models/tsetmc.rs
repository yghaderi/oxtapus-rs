use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
struct OptionMarketWatchRow {
    #[serde(alias = "insCode_P")]
    ins_code_p: String,
    #[serde(alias = "insCode_C")]
    ins_code_c: String,
    #[serde(alias = "contractSize")]
    contract_size: u64,
    #[serde(alias = "uaInsCode")]
    ins_code_ua: String,
    #[serde(alias = "lVal18AFC_P")]
    symbol_p: String,
    #[serde(alias = "lVal30_P")]
    name_p: String,
    #[serde(alias = "zTotTran_P")]
    trade_count_p: u64,
    #[serde(alias = "qTotTran5J_P")]
    traded_vol_p: u64,
    #[serde(alias = "qTotCap_P")]
    traded_val_p: f64,
    #[serde(alias = "notionalValue_P")]
    notional_value_p: f64,
    #[serde(alias = "pClosing_P")]
    final_p: u64,
    #[serde(alias = "priceYesterday_P")]
    y_final_p: u32,
    #[serde(alias = "oP_P")]
    open_interest_p: u32,
    #[serde(alias = "pDrCotVal_P")]
    close_p: u32,
    #[serde(alias = "lval30_UA")]
    symbol_ua: String,
    #[serde(alias = "pClosing_UA")]
    final_ua: u32,
    #[serde(alias = "priceYesterday_UA")]
    y_final_ua: u32,
    #[serde(alias = "beginDate")]
    begin_date: String,
    #[serde(alias = "endDate")]
    end_date: String,
    #[serde(alias = "strikePrice")]
    strike_price: u32,
    #[serde(alias = "remainedDay")]
    t: u32,
    #[serde(alias = "pDrCotVal_C")]
    close_c: u32,
    #[serde(alias = "oP_C")]
    open_interest_c: u64,
    #[serde(alias = "pClosing_C")]
    final_c: u32,
    #[serde(alias = "priceYesterday_C")]
    y_final_c: u32,
    #[serde(alias = "notionalValue_C")]
    notional_value_c: f64,
    #[serde(alias = "qTotCap_C")]
    traded_val_c: f64,
    #[serde(alias = "qTotTran5J_C")]
    traded_vol_c: u64,
    #[serde(alias = "zTotTran_C")]
    trade_count: u64,
    #[serde(alias = "lVal30_C")]
    name_c: String,
    #[serde(alias = "lVal18AFC_C")]
    symbol_c: String,
    #[serde(alias = "pMeDem_P")]
    bid_price_p: u32,
    #[serde(alias = "qTitMeDem_P")]
    bid_vol_p: u64,
    #[serde(alias = "pMeOf_P")]
    ask_price_p: u32,
    #[serde(alias = "qTitMeOf_P")]
    ask_vol_p: u64,
    #[serde(alias = "pMeDem_C")]
    bid_price_c: u32,
    #[serde(alias = "qTitMeDem_C")]
    bid_vol_c: u64,
    #[serde(alias = "pMeOf_C")]
    ask_price_c: u32,
    #[serde(alias = "qTitMeOf_C")]
    ask_vol_c: u64,
    // #[serde= ""]
    // "yesterdayOP_C": 45721,
    // #[serde= ""]
    // "yesterdayOP_P": 15289
}
#[derive(Serialize, Deserialize, Debug)]
pub struct OptionMarketWatch {
    #[serde(alias = "instrumentOptMarketWatch")]
    records: Vec<OptionMarketWatchRow>,
}