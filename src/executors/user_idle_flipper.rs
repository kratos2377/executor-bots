use vortex_sdk::{AccountProvider, VortexDexClient};



pub struct UserIdleFlipperExecutor<T: AccountProvider> {
    pub name: String,
    pub dry_run: bool,
    pub run_once: bool,
    pub default_interval_ms: u64, // Default interval in milliseconds

    pub vortex_client: VortexDexClient<T>,
    pub lookup_table_account: Option<AddressLookupTableAccount>, // Optional lookup table account
    pub interval_ids: Vec<u128>, // Assuming NodeJS_Timer is defined elsewhere
    pub user_map: UserMap, // Assuming UserMap is defined elsewhere
    pub blockhash_subscriber: BlockhashSubscriber, // Assuming BlockhashSubscriber is defined elsewhere

    pub watchdog_timer_mutex: Mutex<()>, // Mutex for watchdog timer
    pub watchdog_timer_last_pat_time: u128, //
}