use std::{collections::{btree_map::Range, HashMap}, net::SocketAddr, sync::{atomic::Ordering, Arc}};

use axum::{response::IntoResponse, routing::get, Router};
use conf::{config_types::ServerConfiguration, configuration};
use event_queue::EventQueue;
use executor::BetSettleExecutor;
use model::GameBetSettleKafkaPayload;
use rdkafka::{consumer::StreamConsumer, Message};
use serde_json::json;
use tokio::{spawn, sync::Mutex, task::JoinHandle};
use tracing::{error, warn};
use crate::configuration::Configuration;


pub mod conf;
pub mod kafka;
pub mod executor;
pub mod errors;
pub mod event_queue;
pub mod model;

pub const START_GAME_SETTLE_EVENT: &str = "start_game_settle_game_event";

#[tokio::main]
async fn main()   {
    let hy_config = configuration::Configuration::load().unwrap();
  //  dotenv().ok();

  let kafka_consumer = kafka::consumer::init_consumers(&hy_config.kafka).unwrap();
  let kafka_producer = kafka::producer::create_new_kafka_producer(&hy_config.kafka).unwrap();

  let mut event_queue = Box::leak(Box::new(EventQueue::new()));

  for ind in 0..hy_config.executors_config.number_of_executors {

    let executor = BetSettleExecutor::new(kafka_producer.clone(), ind.clone() as u32, event_queue.event_queue_sender.clone());

 
    event_queue.add_new_executor(ind.clone(), executor.executor_event_sender);
  }




  let game_bet_settlers_handle = init_game_bet_settle_consumers(event_queue, &hy_config, kafka_consumer);


  start_web_server(&hy_config.server, vec![game_bet_settlers_handle])
    .await


}



async fn start_web_server(
  config: &ServerConfiguration,
  shutdown_handles: Vec<JoinHandle<()>>,
) {
  // Initialize routing
  let routing = init_routing();

  // Start server
  let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
  tracing::info!("listening on {addr}");

  let listener = tokio::net::TcpListener::bind("127.0.0.1:3030")
  .await
  .unwrap();
println!("listening on {}", listener.local_addr().unwrap());
axum::serve(listener, routing.into_make_service_with_connect_info::<SocketAddr>()).with_graceful_shutdown(shutdown_signal(shutdown_handles)).await.unwrap();

  // Shutdown tracing provider
}


pub async fn shutdown_signal(shutdown_handles: Vec<JoinHandle<()>>) {
  let ctrl_c = async {
      tokio::signal::ctrl_c()
          .await
          .expect("Initialization of Ctrl+C handler failed");
  };

  #[cfg(unix)]
  let terminate = async {
      tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
          .expect("Initialization of signal handler failed")
          .recv()
          .await;
  };

  #[cfg(not(unix))]
  let terminate = std::future::pending::<()>();

  tokio::select! {
      _ = ctrl_c => {},
      _ = terminate => {},
  }

  for handle in shutdown_handles {
      handle.abort();
  }
}


pub async fn health() -> impl IntoResponse {
  axum::Json(json!({ "status" : "UP" }))
}


fn init_routing() -> Router {
  let base_router = Router::new().route("/health", get(health));

  return base_router;

}


fn init_game_bet_settle_consumers(
  event_queue: &'static mut EventQueue,
  config: &Configuration,
  kafka_consumers: HashMap<String, StreamConsumer>,
) -> JoinHandle<()> {

  let mut kafka_joins: Vec<JoinHandle<()>> = vec![];

  for (key_topic , value) in kafka_consumers.into_iter() {
     let kf_join =  listen(
        event_queue,
          config,
          value,
          key_topic
      );

      kafka_joins.push(kf_join);
  }

  let join_handle = spawn(async move {
      for handle in kafka_joins {
          handle.await.unwrap();
      }
  });

  return join_handle;
  

}


pub fn listen(
  event_queue: &'static EventQueue,
  config: &Configuration,
  stream_consumer: StreamConsumer,
  key_topic: String,
) -> JoinHandle<()> {
  let topic = key_topic.clone();

  // Start listener
  tokio::spawn(async move {
      do_listen( event_queue,&stream_consumer, topic ).await;
  })
}

pub async fn do_listen(
  event_queue: &'static EventQueue,
  stream_consumer: &StreamConsumer,
  topic_name: String,
) {


  loop {
    if event_queue.get_len() < 5000 {
          match stream_consumer.recv().await {
            Err(e) => warn!("Error: {}", e),
            Ok(message) => {

            let topic = message.topic();
            let payload = String::from_utf8(message.payload().unwrap().to_vec()).unwrap();

            match topic {

              START_GAME_SETTLE_EVENT=> {
                let game_bet_payload = serde_json::from_str(&payload);

                if game_bet_payload.is_err() {
                  error!("Some error occured while parsing string to GameBetKafkaPayload")
                } else {
                  let game_bet_convertred_record: GameBetSettleKafkaPayload = game_bet_payload.unwrap();


                  //later add logic that if push is unsuccessful that publish fail event back
                 let _ =  event_queue.push(game_bet_convertred_record);

              }

            }

        
                _ => {
                    println!("No topics found")
                  }

            }

                
        }
        }
    }
}

}


