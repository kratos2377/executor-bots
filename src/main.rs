use std::collections::btree_map::Range;

use conf::configuration;
use event_queue::EventQueue;
use executor::BetSettleExecutor;



pub mod conf;
pub mod kafka;
pub mod executor;
pub mod errors;
pub mod event_queue;
pub mod model;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>  {
    let hy_config = configuration::Configuration::load().unwrap();
  //  dotenv().ok();

  let kafka_consumer = kafka::consumer::init_consumers(&hy_config.kafka).unwrap();
  let kafka_producer = kafka::producer::create_new_kafka_producer(&hy_config.kafka).unwrap();

  let mut event_queue = EventQueue::new();

  for ind in 0..hy_config.executors_config.number_of_executors {

    let executor = BetSettleExecutor::new(kafka_producer.clone(), ind.clone() as u32, event_queue.event_queue_sender.clone());

 
    event_queue.add_new_executor(ind.clone(), executor.executor_event_sender);
  }

    Ok(())
}

