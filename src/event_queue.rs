use std::sync::{atomic::{AtomicBool, AtomicU64}, RwLock};

use tokio::sync::mpsc::{Receiver, Sender};

use crate::model::GameBetSettleKafkaPayload;




pub struct EventQueue {

    pub events_queue: Vec<GameBetSettleKafkaPayload>,
    pub head: u32,
    pub tail: u32,
    pub len: u32,
    pub executors_senders: Vec<Sender<GameBetSettleKafkaPayload>>,
    pub executors_queue: Vec<String>,
    pub executors_order_listener: Receiver<String>,
    pub event_queue_sender: Sender<String>
}


impl EventQueue {

    pub fn new() -> Self {

        // Add config support to initialize queue with config mentioned

        let (sn , rn) = tokio::sync::mpsc::channel::<String>(100);

        Self {
            events_queue: Vec::with_capacity(6000),
            head: 0,
            tail: 0,
            len: 0,
            executors_senders: Vec::with_capacity(20),
            //Change this to a lock free concurrent queue
            executors_queue: Vec::with_capacity(30),
            executors_order_listener: rn,
            event_queue_sender: sn,
        }

    }

    pub fn add_new_executor(&mut self , index: usize , exec: Sender<GameBetSettleKafkaPayload>) {

            self.executors_senders[index] = exec;
    }

}