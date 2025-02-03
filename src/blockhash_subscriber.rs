use std::{
    collections::VecDeque,
    sync::{Arc, Mutex, RwLock},
    time::Duration,
};

use log::warn;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::hash::Hash;
use tokio::sync::oneshot;
use crate::types::UnsubHandle;

pub struct BlockhashSubscriber {
    refresh_frequency: Duration,
    last_twenty_hashes: Arc<RwLock<VecDeque<Hash>>>,
    rpc_client: Arc<RpcClient>,
    unsub: Mutex<Option<UnsubHandle>>,
}

impl BlockhashSubscriber {
    /// Create a new blockhash subscriber
    /// It must be started by calling `subscribe()`
    pub fn new(refresh_frequency: Duration, rpc_client: Arc<RpcClient>) -> Self {
        BlockhashSubscriber {
            last_twenty_hashes: Arc::new(RwLock::new(VecDeque::with_capacity(20))),
            rpc_client: Arc::clone(&rpc_client),
            refresh_frequency,
            unsub: Mutex::default(),
        }
    }

    /// Start the blockhash subscriber task
    pub fn subscribe(&self) {
        let (unsub_tx, mut unsub_rx) = oneshot::channel();
        {
            let mut guard = self.unsub.try_lock().expect("uncontested");
            if guard.is_some() {
                return;
            }
            guard.replace(unsub_tx);
        }

        tokio::spawn({
            let rpc_client = Arc::clone(&self.rpc_client);
            let last_twenty_hashes = Arc::clone(&self.last_twenty_hashes);
            let mut refresh = tokio::time::interval(self.refresh_frequency);

            let max_attempts = 3;
            let mut attempts = 0;
            async move {
                loop {
                    let _ = refresh.tick().await;
                    match rpc_client.get_latest_blockhash().await {
                        Ok(blockhash) => {
                            attempts = 0;
                            let mut hashes = last_twenty_hashes.write().expect("acquired");
                            hashes.push_back(blockhash);
                            if hashes.len() > 20 {
                                let _ = hashes.pop_front();
                            }
                        }
                        Err(err) => {
                            warn!("blockhash subscriber missed update: {err:?}");
                            attempts += 1;
                            if attempts > max_attempts {
                                panic!("unable to fetch blockhash");
                            }
                        }
                    }

                    if unsub_rx.try_recv().is_ok() {
                        warn!("unsubscribing from blockhashes");
                        break;
                    }
                }
            }
        });
    }

    /// Return most recent polled blockhash
    pub fn get_latest_blockhash(&self) -> Option<Hash> {
        let lock = self.last_twenty_hashes.read().expect("acquired");
        lock.back().copied()
    }

    /// Return oldest valid blockhash (more likely finalized)
    pub fn get_valid_blockhash(&self) -> Option<Hash> {
        let lock = self.last_twenty_hashes.read().expect("acquired");
        lock.front().copied()
    }

    /// Stop the blockhash subscriber task, if it exists    ``
    pub fn unsubscribe(&self) {
        let mut guard = self.unsub.lock().expect("uncontested");
        if let Some(unsub) = guard.take() {
            if unsub.send(()).is_err() {
                log::error!("couldn't unsubscribe");
            }
        }
    }
}