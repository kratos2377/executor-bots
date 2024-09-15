

pub mod utils;
pub mod errors;
pub mod types;
pub mod executors;
pub mod vortex_price_feed_subscriber;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>  {
    println!("Hello, world!");
    Ok(())
}
