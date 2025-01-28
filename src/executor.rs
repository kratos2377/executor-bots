
use rdkafka::producer::FutureProducer;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::model::{EventRecord, GameBetSettleKafkaPayload};


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
}


impl BetSettleExecutor {
    pub fn new(producer: FutureProducer , index: u32 , event_queue_sender: Sender<EventRecord>) -> Self {
        let (exsn , exrn) = tokio::sync::mpsc::channel::<GameBetSettleKafkaPayload>(10);

        Self { 
            
        executor_id: "BetSettleExecutor_".to_string() + &nano_id::base64::<21>(),
         producer: producer,
         event_queue_sender: event_queue_sender, 
         executor_index: index, 
        executor_event_reciever: exrn,
        executor_event_sender: exsn ,

        }
    }
  
}