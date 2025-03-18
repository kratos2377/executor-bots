use std::{collections::{btree_map::Range, HashMap}, env, fs::File, net::SocketAddr, path::PathBuf, str::FromStr, sync::{atomic::Ordering, Arc}};

use axum::{response::IntoResponse, routing::get, Router};
use conf::{config_types::ServerConfiguration, configuration};
use constants::{EXECUTOR_GAME_OVER_STATUS_SETTLED, GAME_BET_SETTLED, GAME_BET_SETTLED_ERROR, SOLANA_DEVNET_URL, STAKE_TIME_OVER_RESULT};
use event_queue::EventQueue;
use executor::BetSettleExecutor;
use log::info;
use model::{EventQueueRecords, EventRecord, ExecutorGameOverEvent, ExecutorGameStakeTimeOverEvent, GameBetSettleKafkaPayload, GameStakeStatusChangeEvent, GameStatusChangeEvent, GameUserBetSettleEvent, CONSUMER_GROUP_BET_EVENT_ADD, EXECUTOR_INDEX_ADD, GAME_OVER_EVENT, GAME_STAKE_TIME_OVER_EVENT};
use rdkafka::{consumer::StreamConsumer, message::ToBytes, Message};
use serde_json::json;
use solana_client::{nonblocking::rpc_client::RpcClient, rpc_config::RpcSendTransactionConfig};
use solana_sdk::{commitment_config::CommitmentLevel, pubkey::Pubkey, signature::Keypair, signer::{EncodableKey, Signer}};
use tokio::{spawn, sync::{mpsc::{Receiver, Sender}, Mutex}, task::JoinHandle};
use tracing::{error, warn};
use transaction::{get_change_game_over_status_instruction, get_settle_all_games_instruction, get_settle_bet_instruction_for_invalid_game};
use types::VortexSdkError;
use utils::get_vortex_signer_account;
use uuid::Uuid;
use crate::configuration::Configuration;


pub mod conf;
pub mod kafka;
pub mod executor;
pub mod errors;
pub mod event_queue;
pub mod model;
pub mod constants;
pub mod transaction;
pub mod vortex_idl;
pub mod utils;
pub mod types;
pub mod wallet;
pub mod blockhash_subscriber;
pub mod executor_rpc_client;
pub mod remaining_account;
pub mod logging_tracing;

pub const START_GAME_SETTLE_EVENT: &str = "start_game_settle_game_event";
pub const EXECUTOR_GAME_OVER_EVENT: &str = "executor_game_over_event";
pub const EXECUTOR_GAME_STAKE_TIME_OVER_EVENT: &str = "executor_game_stake_time_over_event";

#[tokio::main]
async fn main()   {
    let hy_config = configuration::Configuration::load().unwrap();
  //  dotenv().ok();

  let kafka_consumer = kafka::consumer::init_consumers(&hy_config.kafka).unwrap();

  let mut event_queue = Box::leak(Box::new(EventQueue::new()));

  let event_queue_sender_clone = event_queue.event_queue_sender.clone();
  
  let mut hyperion_handles = vec![];


  let file = File::open(hy_config.executors_config.keypair_path.clone()).unwrap();
  let keypair_bytes: Vec<u8> = serde_json::from_reader(file).unwrap();
  
  // Create keypair from bytes

  for ind in 0..hy_config.executors_config.number_of_executors {

    let vortex_keypair = Keypair::from_bytes(&keypair_bytes).unwrap();

    // let each executor have its own kafka producer

    let kafka_producer = kafka::producer::create_new_kafka_producer(&hy_config.kafka).unwrap();
    let mut executor = BetSettleExecutor::new(kafka_producer, ind.clone() as u32, event_queue.event_queue_sender.clone(),
                                RpcClient::new(SOLANA_DEVNET_URL.to_string()) , vortex_keypair  ).await;

    println!("Initialized executor={:?} with index={:?}" , executor.executor_id.clone() , executor.executor_index.clone());
    event_queue.add_new_executor(ind.clone(), executor.executor_event_sender.clone());


    let executor_handle = tokio::spawn(async move {
     loop {
       match executor.executor_event_reciever.try_recv() {

        Ok(event_record) =>  {

          if event_record.game_settle_record.is_some() {
            let bet_settlement_event = event_record.game_settle_record.unwrap();
          // session id will always be of length 21 so we can enforce the length
          
            println!("Starting execution to SettleBet for game_id={:?} session_id={:?} by executor={:?}" , bet_settlement_event.game_id.clone() , bet_settlement_event.session_id.clone() , executor.executor_id);
            if bet_settlement_event.session_id.len() == 21 {
              let game_uuid = Uuid::parse_str(&bet_settlement_event.game_id).unwrap();
              let user_uuid =Uuid::parse_str(&bet_settlement_event.user_id).unwrap();
              let user_betting_onuuid = Uuid::parse_str(&bet_settlement_event.user_betting_on).unwrap();

              let game_id_bytes = game_uuid.as_bytes();
              let user_id_bytes = user_uuid.as_bytes();
              let user_betting_on_bytes = user_betting_onuuid.as_bytes();
              
            let session_id_bytes = bet_settlement_event.session_id.as_bytes().try_into().unwrap();
            
    
              let user_bet_wallet_key = Pubkey::from_str( &bet_settlement_event.user_wallet_key).unwrap();
              //Add Solana instruction creator 
                let tx = if bet_settlement_event.is_valid {

                 if !bet_settlement_event.winner_id.is_empty() {
                  let winner_id_bytes = Uuid::parse_str(&bet_settlement_event.winner_id).unwrap().to_bytes_le();
    
                  get_settle_all_games_instruction(*executor.vortex_exec_client.wallet().authority() , game_id_bytes , user_id_bytes , user_betting_on_bytes , session_id_bytes , winner_id_bytes , user_bet_wallet_key ).await
                 } else {
                  //stalemate case
                  get_settle_bet_instruction_for_invalid_game(*executor.vortex_exec_client.wallet().authority() , game_id_bytes , user_id_bytes , user_betting_on_bytes , session_id_bytes  , user_bet_wallet_key, true ).await

                 }
    
                } else {

                  // We have to send one more field if game is invalid whether the instruction is for player or simple user
                  get_settle_bet_instruction_for_invalid_game(*executor.vortex_exec_client.wallet().authority() , game_id_bytes , user_id_bytes , user_betting_on_bytes , session_id_bytes  , user_bet_wallet_key, true ).await
                };
  
  
                
               let res =  if tx.is_ok() {
                    let tx_record = tx.unwrap();
  
             executor.vortex_exec_client.sign_and_send_with_config( tx_record, None, RpcSendTransactionConfig{ skip_preflight: false, 
                      preflight_commitment: Some(CommitmentLevel::Processed), encoding: None, max_retries: None, min_context_slot: None }).await
  
  
                } else {
                  Err(VortexSdkError::ErrorWhileParsingTransactionRecord)
                };
  
  
            // Add success/failure producer logic
        
  
  
           let (kafka_payload_event,kafka_topic) = if res.is_err() {
            println!("Recieved error while executing transaction");
            println!("{:?}" , res.err().unwrap());
            
              // If error we will publish error event to kafka
              let kafka_event  =   GameUserBetSettleEvent { game_id: bet_settlement_event.game_id,
                 session_id: bet_settlement_event.session_id, user_id: bet_settlement_event.user_id, winner_id: bet_settlement_event.winner_id, is_game_valid: bet_settlement_event.is_valid, is_error: true };
            
                (kafka_event , GAME_BET_SETTLED_ERROR)
            } else {
             let kafka_event =  GameUserBetSettleEvent { game_id: bet_settlement_event.game_id,
                session_id: bet_settlement_event.session_id, user_id: bet_settlement_event.user_id, winner_id: bet_settlement_event.winner_id, is_game_valid: bet_settlement_event.is_valid, is_error: false };
  
                (kafka_event , GAME_BET_SETTLED)
            };
  
            let _ = executor.produce_event_to_kafka_topic(vec![kafka_payload_event] , vec![] , kafka_topic ).await;
    
            }
  
  
  
            //Once event is producer let event queue know the executor is available to pick up new task
            let new_event_record = EventRecord { payload: executor.executor_index.to_string(), event_type: EXECUTOR_INDEX_ADD.to_string() };
            let _ = executor.event_queue_sender.send(new_event_record).await;
  
          } else if event_record.game_stake_time_over_record.is_some() {


            let game_over_record_event = event_record.game_stake_time_over_record.unwrap();
            // session id will always be of length 21 so we can enforce the length
            
              println!("Starting execution to set stake status for game_id={:?} session_id={:?} by executor={:?} to over" , game_over_record_event.game_id.clone() , game_over_record_event.session_id.clone() , executor.executor_id);
              if game_over_record_event.session_id.len() == 21 {
                let game_uuid = Uuid::parse_str(&game_over_record_event.game_id).unwrap();
    
                let game_id_bytes = game_uuid.as_bytes();
                
              let session_id_bytes = game_over_record_event.session_id.as_bytes().try_into().unwrap();
      
                //Add Solana instruction creator 
                  let tx = get_change_game_over_status_instruction(*executor.vortex_exec_client.wallet().authority(), &game_id_bytes, session_id_bytes).await;
    
    
                  
                 let res =  if tx.is_ok() {
                      let tx_record = tx.unwrap();
    
               executor.vortex_exec_client.sign_and_send_with_config( tx_record, None, RpcSendTransactionConfig{ skip_preflight: false, 
                        preflight_commitment: Some(CommitmentLevel::Processed), encoding: None, max_retries: None, min_context_slot: None }).await
    
    
                  } else {
                    Err(VortexSdkError::ErrorWhileParsingTransactionRecord)
                  };
    
    
              // Add success/failure producer logic
          
                  // Kafka Event will be generated to tell whether instruction was successful or not 
                  // If yes only then ExecutorBots will start getting instructions for SettleEvents
    
             let (kafka_payload_event , kafka_topic) = if res.is_err() {
              println!("Recieved error while executing transaction for game over event");
              println!("{:?}" , res.err().unwrap());
                // If error we will publish error event to kafka
                let kafka_event =   GameStakeStatusChangeEvent { game_id: game_over_record_event.game_id.clone(),
                   session_id: game_over_record_event.session_id.clone(),
                    is_error: true };
              
                 ( kafka_event, STAKE_TIME_OVER_RESULT)
              } else {
    
               let kafka_event =   GameStakeStatusChangeEvent { game_id: game_over_record_event.game_id.clone(),
                session_id: game_over_record_event.session_id.clone(),
                 is_error: false };
    
                 ( kafka_event , STAKE_TIME_OVER_RESULT)
              };
    
              let _ = executor.produce_event_to_kafka_topic(vec![] , vec![kafka_payload_event] , kafka_topic).await;
      
              }
    
    
    
              //Once event is producer let event queue know the executor is available to pick up new task
              let new_event_record = EventRecord { payload: executor.executor_index.to_string(), event_type: EXECUTOR_INDEX_ADD.to_string() };
              let _ = executor.event_queue_sender.send(new_event_record).await;

          } else {
            println!("Invalid EventRecordReceieved")
          }


        
        }

        Err(err) => {}
      }
    }
    });


    hyperion_handles.push(executor_handle);

  }






  // THis handle will listen to all events coming to EventQueue




  let event_queue_events_listener = tokio::spawn({
    let mut event_records_listener = Box::leak(Box::new(event_queue.executors_order_listener.take().unwrap()));
    async {
      loop {
        let event_recv = event_records_listener.try_recv();

        if event_recv.is_ok() {
          let event_record = event_recv.unwrap();
          match  event_record.event_type.as_str() {
            CONSUMER_GROUP_BET_EVENT_ADD => {
                let parsed_payload = serde_json::from_str(&event_record.payload);


                if parsed_payload.is_ok() {
                  let game_bet_record: GameBetSettleKafkaPayload = parsed_payload.unwrap();
                  let push_res = event_queue.push(EventQueueRecords { game_settle_record: Some(game_bet_record), game_stake_time_over_record: None });

                  if push_res.is_err() {
                    println!("Error while pushing event in event queue");
                  } else {
                    println!("Successfully pushed settle bet event in event queue");
                  }
                } else {
                  println!("Error while parsing gamebetsettlekafkapayload event");
                }
            },


            EXECUTOR_INDEX_ADD => {
              let parsed_payload = event_record.payload.parse::<usize>();


              if parsed_payload.is_ok() {
                let parsed_ind = parsed_payload.unwrap();
                let exec_res = event_queue.executors_queue.push(parsed_ind);

                if exec_res.is_err() {
                  println!("Error while pushing executor index");
                } else {
                  println!("Successfully pushed executor index");
                }
              }
            },

            GAME_STAKE_TIME_OVER_EVENT => {
              let parsed_payload = serde_json::from_str(&event_record.payload);


              if parsed_payload.is_ok() {
                let executor_game_stake_time_over_record: ExecutorGameStakeTimeOverEvent = parsed_payload.unwrap();
                let push_res = event_queue.push(EventQueueRecords { game_settle_record: None, game_stake_time_over_record: Some(executor_game_stake_time_over_record) });

                if push_res.is_err() {
                  println!("Error while pushing game_over_event event in event queue");
                } else {
                  println!("Successfully pushed game stake time over event in event queue");
                }
              } else {
                println!("Error while parsing gamestaketimeover event");
              }
            }

            _ => {}
          }
        }
    }
    }

  });


      //Check queue and send events to executors
      let send_bet_to_executors = tokio::spawn(async  {
        loop {
          
          if event_queue.executors_queue.len() > 0 {
    
            if event_queue.get_len() > 0   {
              let exec_ind_rec = event_queue.executors_queue.pop().unwrap();
              let bet_event = event_queue.pop();
      
              if bet_event.is_some() {
                println!("Received a event from event queue rec. Sending it to executor with index: {:?}" , exec_ind_rec);
                let bet_event_record = bet_event.unwrap();
               let sender =    event_queue.executors_senders.get(exec_ind_rec).unwrap();
      
                let _ = sender.send(bet_event_record).await;
      
              }
            }
          } 
        }
      });
    



  let game_bet_settlers_handle = init_game_bet_settle_consumers(event_queue_sender_clone, &hy_config, kafka_consumer);

  hyperion_handles.push(event_queue_events_listener);
  hyperion_handles.push(send_bet_to_executors);
  hyperion_handles.push(game_bet_settlers_handle);


  start_web_server(&hy_config.server, hyperion_handles)
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
  axum::Json(json!({ "Executor-Bots status" : "UP" }))
}


fn init_routing() -> Router {
  let base_router = Router::new().route("/api/v1/health", get(health));

  return base_router;

}


fn init_game_bet_settle_consumers(
  consumer_event_sender: Sender<EventRecord>,
  config: &Configuration,
  kafka_consumers: HashMap<String, StreamConsumer>,
) -> JoinHandle<()> {

  let mut kafka_joins: Vec<JoinHandle<()>> = vec![];

  for (key_topic , value) in kafka_consumers.into_iter() {
     let kf_join =  listen(
        consumer_event_sender.clone(),
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
  consumer_event_sender: Sender<EventRecord>,
  config: &Configuration,
  stream_consumer: StreamConsumer,
  key_topic: String,
) -> JoinHandle<()> {
  let topic = key_topic.clone();

  // Start listener
  tokio::spawn(async move {
      do_listen( consumer_event_sender,&stream_consumer, topic ).await;
  })
}

pub async fn do_listen(
  consumer_event_sender: Sender<EventRecord>,
  stream_consumer: &StreamConsumer,
  topic_name: String,
) {


  loop {
    // Need to somehow add logic to consume events only when size of event_queue <5000
          match stream_consumer.recv().await {
            Err(e) => warn!("Error: {}", e),
            Ok(message) => {

            let topic = message.topic();
            let payload = String::from_utf8(message.payload().unwrap().to_vec()).unwrap();

            match topic {

              START_GAME_SETTLE_EVENT=> {
                println!("Received start game settle events");
                println!("With payload: {:?}" , payload.clone());
                let game_bet_payload_event_record = EventRecord {
                    payload,
                    event_type: CONSUMER_GROUP_BET_EVENT_ADD.to_string(),
                };


                  //later add logic that if push is unsuccessful that publish fail event back
                 let publish_res =  consumer_event_sender.send(game_bet_payload_event_record).await;

                  if publish_res.is_err() {
                    println!("error while publishing to event queue");
                  } else {
                    println!("Successfully generated events");
                  }
              

            },

            EXECUTOR_GAME_STAKE_TIME_OVER_EVENT => {
              println!("Received executor game over event");
              println!("With payload: {:?}" , payload.clone());
              let exec_game_over_event = EventRecord {
                  payload,
                  event_type: GAME_STAKE_TIME_OVER_EVENT.to_string(),
              };


                //later add logic that if push is unsuccessful that publish fail event back
               let publish_res =  consumer_event_sender.send(exec_game_over_event).await;

                if publish_res.is_err() {
                  println!("error while publishing to event queue");
                } else {
                  println!("Successfully generated events");
                }
            },

        
                _ => {
                    println!("No topics found")
                  }

            }

                
        }
        }
    
}

}
