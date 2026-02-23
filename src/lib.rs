mod client;
mod error;
mod tsetmc;

pub use client::TsetmcClient;
pub use error::{Error, Result};

pub mod models {
    pub use crate::tsetmc::models::History;
    pub use crate::tsetmc::models::HistoryRecord;
    pub use crate::tsetmc::models::OptionMarketWatch;
    pub use crate::tsetmc::models::OptionMarketWatchRow;
}
