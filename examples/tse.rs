use std::error::Error;
use oxtapus::tsetmc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tsetmc::tse_option_mw().await?;
    Ok(())
}
