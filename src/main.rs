use conf::configuration;



pub mod conf;
pub mod kafka;
pub mod executor;
pub mod errors;
pub mod event_queue;
pub mod model;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>  {
    let config = configuration::Configuration::load().unwrap();
  //  dotenv().ok();

  


    Ok(())
}

