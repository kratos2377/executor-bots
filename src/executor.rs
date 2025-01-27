use std::{sync::atomic::Ordering, time::Duration};

use futures::{stream, StreamExt}; 
use rdkafka::producer::FutureProducer;
use tokio::{sync::mpsc::{Receiver, Sender}, time::{self, Instant, Interval, Timeout}};

use crate::{event_queue::EventQueue, model::GameBetSettleKafkaPayload};


pub struct BetSettleExecutor {
    pub executor_id: String,
    pub producer: FutureProducer,
    pub event_queue_sender: Sender<String>,
    pub executor_index: u32,
    pub executor_event_reciever: Receiver<GameBetSettleKafkaPayload>,
    pub executor_event_sender: Sender<GameBetSettleKafkaPayload>,
}


impl BetSettleExecutor {
    pub fn new(producer: FutureProducer , index: u32 , event_queue_sender: Sender<String>) -> Self {
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