use futures_util::lock::Mutex;
use num_bigint::BigInt;
use vortex_sdk::{common::priority_fee::PriorityFeeSubscriberMap, slot_subscriber::SlotSubscriber, usermap::UserMap, AccountProvider, AddressLookupTableAccount, VortexDexClient};

use crate::global_config::GlobalConfig;




pub struct UserPnLSettler<T: AccountProvider> {
    pub name: String,
    pub dry_run: bool,
    pub run_once: bool,
    pub default_interval_ms: u64, // Default interval in milliseconds

    pub vortex_client: VortexDexClient<T>,
    pub slot_subscriber: SlotSubscriber,
    pub global_config: GlobalConfig, // Assuming GlobalConfig is defined elsewhere
    pub lookup_table_account: Option<AddressLookupTableAccount>, // Optional lookup table account
    pub interval_ids: Vec<u128>, // Assuming NodeJS_Timer is defined elsewhere
    pub user_map: UserMap, // Assuming UserMap is defined elsewhere
    pub priority_fee_subscriber_map: Option<PriorityFeeSubscriberMap>, // Optional subscriber map
    pub in_progress: bool, // Boolean to indicate progress
    pub market_indexes: Vec<u32>, // Vector of market indexes
    pub min_pnl_to_settle: BigInt, // Assuming BN is defined elsewhere
    pub max_users_to_consider: u32, // Maximum users to consider

    pub watchdog_timer_mutex: Mutex<()>, // Mutex for watchdog timer
    pub watchdog_timer_last_pat_time: u128,
}