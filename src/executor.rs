
use std::time::Duration;

use rdkafka::{error::KafkaError, producer::{FutureProducer, FutureRecord, Producer}, util::Timeout};
use solana_client::nonblocking::rpc_client::{self, RpcClient};
use solana_sdk::signature::Keypair;
use tokio::sync::mpsc::{Receiver, Sender};
use futures_util::future;
use crate::{constants::{GAME_BET_SETTLED, GAME_BET_SETTLED_ERROR}, executor_rpc_client::{Context, VortexExecutorClient}, model::{EventRecord, GameBetSettleKafkaPayload, GameUserBetSettleEvent}, wallet::Wallet};


pub struct BetSettleExecutor {
    pub executor_id: String,
    pub producer: FutureProducer,
    // This will send event to event_queue to add to the executors_order queue
    pub event_queue_sender: Sender<EventRecord>,
    pub executor_index: u32,
    // this will listen to events that need to be settled
    pub executor_event_reciever: Receiver<GameBetSettleKafkaPayload>,
    // this will go in event_queue vec so that event_queue can send events to executors
    pub executor_event_sender: Sender<GameBetSettleKafkaPayload>,

    //Vortex Client connection to execute transactions
    pub vortex_exec_client: VortexExecutorClient,
}


impl BetSettleExecutor {
    pub async fn new(producer: FutureProducer , index: u32 , event_queue_sender: Sender<EventRecord> , rpc_client: RpcClient , authority: Keypair) -> Self {
        let (exsn , exrn) = tokio::sync::mpsc::channel::<GameBetSettleKafkaPayload>(10);

        Self { 
            
        executor_id: "BetSettleExecutor_".to_string() + &nano_id::base64::<21>(),
         producer: producer,
         event_queue_sender: event_queue_sender, 
         executor_index: index, 
        executor_event_reciever: exrn,
        executor_event_sender: exsn ,
        vortex_exec_client: VortexExecutorClient::new( rpc_client, Wallet::new(authority)).await.unwrap()

        }
    }


    pub async fn produce_event_to_kafka_topic(&self , game_bet_events: Vec<GameUserBetSettleEvent> ) -> Result<(), KafkaError> {
       
         self.producer.begin_transaction().unwrap();

        let kafka_result = future::try_join_all(game_bet_events.iter().map(|event| async move {
          let publish_topic = if event.is_error {
            GAME_BET_SETTLED_ERROR
          } else   { GAME_BET_SETTLED };
            let converted_string_event = serde_json::to_string(event).unwrap();
            
            let delivery_result = self.producer
            .send(
                FutureRecord::to(&publish_topic)
                        .payload(&converted_string_event)
                        .key("game_bet_settle_result"),
                Duration::from_secs(2),
            )
            .await;
    
        // This will be executed when the result is received.
      //  println!("Delivery status for message {} received", i);
        delivery_result
    
        })
    
        ).await;
    
        match kafka_result {
            Ok(_) => (),
            Err(e) => return Err(e.0.into()),
        }
    
        self.producer.commit_transaction(Timeout::from(Duration::from_secs(2))).unwrap(); 
    
        Ok(())
    }
  
}