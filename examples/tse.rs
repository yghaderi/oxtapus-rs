use oxtapus::tsetmc;
use std::error::Error;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();
    let data = tsetmc::history(vec![String::from("46348559193224090")]).await?;
    println!("{:#?}", data[0].records[0]);
    println!("{:?}", start.elapsed());
    Ok(())
}
