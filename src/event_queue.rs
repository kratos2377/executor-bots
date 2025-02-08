use std::sync::{atomic::{AtomicBool, AtomicU64, Ordering}, Arc, RwLock};

use crossbeam::{queue::ArrayQueue, utils::CachePadded};
use futures::future::Join;
use tokio::{sync::mpsc::{Receiver, Sender}, task::JoinHandle};

use crate::model::{EventRecord, GameBetSettleKafkaPayload, CONSUMER_GROUP_BET_EVENT_ADD, EXECUTOR_INDEX_ADD};




pub struct EventQueue {

    pub events_queue: ArrayQueue<GameBetSettleKafkaPayload>,
    pub executors_senders: Vec<Sender<GameBetSettleKafkaPayload>>,
    pub executors_queue: ArrayQueue<usize>,
    pub executors_order_listener: Option<Receiver<EventRecord>>,
    pub event_queue_sender: Sender<EventRecord>
}


impl EventQueue {

    pub fn new() -> Self {

        // Add config support to initialize queue with config mentioned

        let (sn , rn) = tokio::sync::mpsc::channel::<EventRecord>(10000);

        Self {
            events_queue: ArrayQueue::new(6000),
            executors_senders: Vec::with_capacity(20),
            //Change this to a lock free concurrent queue
            executors_queue: ArrayQueue::new(30),
            executors_order_listener: Some(rn),
            event_queue_sender: sn,
        }

    }

    pub fn add_new_executor(&mut self , index: usize , exec: Sender<GameBetSettleKafkaPayload>) {

            self.executors_senders.push(exec);
    }


    pub fn get_len(&self) -> u64 {
       self.events_queue.len() as u64
    }




    pub fn push(&self, event: GameBetSettleKafkaPayload) -> Result<(), &'static str> {
        self.events_queue.push(event);
        Ok(())
    }

    // Pop an event using atomic operations (immutable `&self`)
    pub fn pop(&self) -> Option<GameBetSettleKafkaPayload> {
       self.events_queue.pop()     
    }

}