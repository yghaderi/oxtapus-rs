use oxtapus::tsetmc;
use std::error::Error;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();
    let data = tsetmc::history().await?;
    println!("{:#?}", data.records[0]);
    println!("{:?}", start.elapsed());
    Ok(())
}
