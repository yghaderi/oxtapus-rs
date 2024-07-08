use std::error::Error;
use serde::{Serialize, Deserialize};
use oxtapus::tsetmc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // let data = tsetmc::tse_option_mw().await?;
    let data = tsetmc::mw().await?;
    println!("{:#?}", data);
    Ok(())
}
