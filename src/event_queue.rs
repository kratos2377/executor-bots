use std::sync::{atomic::{AtomicBool, AtomicU64, Ordering}, Arc, RwLock};

use crossbeam::{queue::ArrayQueue, utils::CachePadded};
use futures::future::Join;
use tokio::{sync::mpsc::{Receiver, Sender}, task::JoinHandle};

use crate::model::{EventRecord, GameBetSettleKafkaPayload, CONSUMER_GROUP_BET_EVENT_ADD, EXECUTOR_INDEX_ADD};




pub struct EventQueue {

    pub events_queue: Vec<GameBetSettleKafkaPayload>,
    pub head: CachePadded<AtomicU64>,
    pub tail: CachePadded<AtomicU64>,
    pub len: u32,
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
            events_queue: Vec::with_capacity(6000),
            head: CachePadded::new(AtomicU64::new(0)),
            tail: CachePadded::new(AtomicU64::new(0)),
            len: 0,
            executors_senders: Vec::with_capacity(20),
            //Change this to a lock free concurrent queue
            executors_queue: ArrayQueue::new(30),
            executors_order_listener: Some(rn),
            event_queue_sender: sn,
        }

    }

    pub fn add_new_executor(&mut self , index: usize , exec: Sender<GameBetSettleKafkaPayload>) {

            self.executors_senders[index] = exec;
    }


    pub fn get_len(&self) -> u64 {
        let head = self.head.load(Ordering::Acquire);
        let tail = self.tail.load(Ordering::Acquire);
        let capacity = self.events_queue.capacity() as u64;

        if tail >= head {
            tail - head
        } else {
            ( head - tail) + 1
        }
    }




    pub fn push(&self, event: GameBetSettleKafkaPayload) -> Result<(), &'static str> {
        let current_tail = self.tail.load(Ordering::Relaxed);
        let next_tail = (current_tail + 1) % self.events_queue.capacity() as u64;
        let current_head = self.head.load(Ordering::Acquire);

        if next_tail == current_head {
            return Err("Buffer full");
        }

        unsafe {
            let slot = self.events_queue.as_ptr().add(current_tail as usize) as *mut _;
            std::ptr::write(slot, event);
        }

        self.tail.store(next_tail, Ordering::Release);
        Ok(())
    }

    // Pop an event using atomic operations (immutable `&self`)
    pub fn pop(&self) -> Option<GameBetSettleKafkaPayload> {
        let current_head = self.head.load(Ordering::Relaxed);
        let current_tail = self.tail.load(Ordering::Acquire);

        if current_head == current_tail {
            return None;
        }

        let event = unsafe {
            let slot = self.events_queue.as_ptr().add(current_head as usize);
            std::ptr::read(slot)
        };

        let next_head = (current_head + 1) % self.events_queue.capacity() as u64;
        self.head.store(next_head, Ordering::Release);
        Some(event)
    }

}