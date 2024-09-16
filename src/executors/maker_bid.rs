use std::{collections::HashMap, sync::Arc, time::Duration};

use futures_util::lock::Mutex;
use vortex_sdk::{dlob::dlob::DLOB, slot_subscriber::SlotSubscriber, usermap::UserMap, AccountProvider, AddressLookupTableAccount, VortexDexClient};

use crate::vortex_price_feed_subscriber::VortexPriceFeedSubscriber;




pub struct MakerBidAskTwapCrank<T: AccountProvider> {
    pub name: String,
    pub dry_run: bool,
    pub run_once: bool,
    pub default_interval_ms: Option<u64>,

    pub global_config: GlobalConfig,

    pub crank_interval_to_market_ids: Option<HashMap<u64, Vec<u64>>>,
    pub crank_interval_in_progress: Option<HashMap<u64, Vec<u64>>>,
    pub all_crank_interval_groups: Option<Vec<u64>>,
    pub max_interval_group: Option<u64>,

    pub slot_subscriber: SlotSubscriber,
    pub vortex_client: VortexDexClient<T>,
    pub interval_ids: Vec<u64>,
    pub user_map: Option<UserMap>,

    pub dlob: DLOB,
    pub lates_dlob_slot: u64,
    pub priority_fee_sub_map: Option<PriorityFeeSubscriberMap>,

    pub watchdog_timer_mutex: Arc<Mutex<u64>>,
    pub watchdog_timer_last_pat_time: Duration,
    pub vortex_price_feed_subscriber: VortexPriceFeedSubscriber,
    pub vortex_price_feed_health_status: bool,
    pub lookup_table_accounts: Vec<AddressLookupTableAccount>

}