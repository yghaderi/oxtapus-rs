use oxtapus::TsetmcClient;
use std::error::Error;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();
    let client = TsetmcClient::new();

    let data = client.option_market_watch().await?;
    println!("{} ردیف آپشن دریافت شد", data.records.len());

    println!("{:#?}", data);
    println!("{:?}", start.elapsed());
    Ok(())
}
