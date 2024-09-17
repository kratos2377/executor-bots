use std::collections::HashMap;

use futures_util::lock::Mutex;
use vortex_sdk::{slot_subscriber::SlotSubscriber, AccountProvider, AddressLookupTableAccount, VortexDexClient};




pub struct UncrossArbExecutor<T: AccountProvider> {
    pub name: String,
    pub dry_run: bool,
    pub default_interval_ms: u64, // Default interval in milliseconds

    pub vortex_env: VortexEnvironmentConfig,
    pub periodic_task_mutex: Mutex<()>, // Mutex for synchronization

    pub vortex_client: VortexDexClient<T>,
    pub lookup_table_account: Option<AddressLookupTableAccount>, // Optional lookup table account
    pub interval_ids: Vec<u64>, // Assuming NodeJS_Timer is defined elsewhere

    pub watchdog_timer_mutex: Mutex<()>, // Mutex for watchdog timer
    pub watchdog_timer_last_pat_time: u128, // Last pat time in milliseconds since epoch

    pub dlob_subscriber: DTLSubscriber,
    pub slot_subscriber: SlotSubscriber,
    pub order_subscriber: OrderSubscriber,
    pub priority_fee_subscriber: PriorityFeeSubscriber,

    pub last_settle_pnl: u128, // Last settle PnL in milliseconds since epoch
    pub throttled_nodes: HashMap<u32, HashMap<String, u32>>, // Nested HashMap for throttled nodes
    pub no_arb_errors: HashMap<u32, HashMap<String, u32>>, 
}