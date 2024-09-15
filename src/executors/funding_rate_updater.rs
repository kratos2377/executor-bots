use std::time::Duration;

use futures_util::lock::Mutex;
use vortex_sdk::{AccountProvider, AddressLookupTableAccount, VortexDexClient};



pub struct FundingRateUpdaterBot<T: AccountProvider> {
    pub name: String,
    pub dry_run: bool,
    pub run_once: bool,
    pub default_interval_ms: u64,
    pub vortex_client: VortexDexClient<T>,
    pub interval_ids: Vec<u64>,
    pub priority_fee_subscriber_map: Option<PriorityFeeSubscriberMap>,
    pub lookup_table_account: Option<AddressLookupTableAccount>,

    pub watchdog_time_mutex: Mutex<u64>,
    pub watchdog_timer_last_pat_time: Duration,
    pub in_progress: bool,
}