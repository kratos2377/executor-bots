use std::{collections::HashMap, sync::Arc, time::Duration};

use futures_util::lock::Mutex;
use lru::LruCache;
use num_bigint::BigInt;
use vortex_contracts::state::user_map::UserStatsMap;
use vortex_sdk::{blockhash_subscriber::BlockhashSubscriber, common::priority_fee::PriorityFeeSubscriber, dlob::dlob_node::NodeToFill, dtl_subscriber::dtl_subscriber::DTLSubscriber, order_subscriber::order_subscriber::OrderSubscriber, usermap::UserMap, AccountProvider, AddressLookupTableAccount, VortexDexClient};

use crate::{global_config::GlobalConfig, metrics::{CounterValue, GaugeValue, HistogramValue, Metrics, RuntimeSpec}, vortex_price_feed_subscriber::VortexPriceFeedSubscriber};

use super::filler::TxType;


pub struct SpotTxSigsToConfirm {
    pub ts: u64,
    pub nodes_filled: Option<NodeToFill>,
    pub fill_tx_id: u64,
    pub tx_type: TxType,
}


pub struct SpotFillerExecutor<'a, T: AccountProvider , S> {

    pub name: String,
    pub dry_run: bool,
    pub default_interval_ms: u64,
    pub vortex_client: VortexDexClient<T>,
    pub global_config: GlobalConfig,
    pub blockhash_subscriber: BlockhashSubscriber,
    pub polling_interval_ms: u64,
    pub fill_tx_id: u64,


    pub dlob_subscriber: Option<DTLSubscriber<T>>,
    pub user_map: UserMap,
    pub order_subscriber: OrderSubscriber<T>,
    pub user_stats_map: UserStatsMap<'a>,

    pub periodic_task_mutex: Arc<Mutex<u64>>,

    pub watchdog_timer_mutex: Arc<Mutex<u64>>,
    pub watchdog_timer_last_pat_time: Duration,
    pub interval_ids: Vec<u64>,
    pub throttled_nodes: HashMap<String, u64>,
    pub triggering_nodes: HashMap<String, u64>,
    pub priority_fee_subscriber: PriorityFeeSubscriber,
    pub revert_on_failure: bool,
    pub simulate_tx_for_cu_estimate: Option<bool>,

    pub pending_tx_sigs_to_confirm: LruCache<String , SpotTxSigsToConfirm>,

    pub expired_nodes_set: LruCache<String , bool>,
    pub confirm_loop_running: bool,
    pub confirm_loop_rate_limit_ts: Duration,

    //for the metrics
    pub metrics_initialized: bool,
    pub metrics_port: Option<u64>,
    pub metrics: Option<Metrics<S>>,
    pub boot_time_ms: Option<u64>,


    pub runtime_spec: RuntimeSpec,
    pub runtime_specs_gauge: Option<GaugeValue<S>>,
    pub try_fill_duration_histogram: Option<HistogramValue<S>>,
    pub est_tx_cu_historgram: Option<HistogramValue<S>>,
    pub simulate_tx_histogram: Option<HistogramValue<S>>,
    pub last_try_fill_time_gauge: Option<GaugeValue<S>>,

    pub mutex_busy_counter: Option<CounterValue<S>>,
    pub sent_txs_counter: Option<CounterValue<S>>,
    pub attempted_triggers_counter: Option<CounterValue<S>>,
    pub landed_txs_counter: Option<CounterValue<S>>,
    pub tx_sim_error_counter: Option<CounterValue<S>>,
    pub pending_tx_sigs_to_confirm_gauge: Option<GaugeValue<S>>,
    pub pending_tx_sigs_loop_rate_limited_counter: Option<CounterValue<S>>,
    pub evicted_pending_tx_sigs_to_confirm_counter: Option<CounterValue<S>>,
    pub expired_nodes_set_size: Option<GaugeValue<S>>,
    pub jito_bundles_accepted_gauge: Option<GaugeValue<S>>,
    pub jito_bundles_simulation_failure_gauge: Option<GaugeValue<S>>,
    pub jito_dropped_bundle_gauge: Option<GaugeValue<S>>,
    pub jito_landed_tips_gauge: Option<GaugeValue<S>>,
    pub jito_bundle_count: Option<GaugeValue<S>>,
    pub clock_subscriber_ts: Option<GaugeValue<S>>,
    pub wall_clock_ts: Option<GaugeValue<S>>,

    pub rebalance_filler: Option<bool>, 

    pub has_enough_sol_to_fill: bool, // Required boolean
    pub min_gas_balance_to_fill: f64, // Assuming this is a floating-point number
    pub rebalance_settled_pnl_threshold: BigInt, // Assuming BigInt is defined elsewhere

    pub price_feed_subscriber: Option<VortexPriceFeedSubscriber>, // Optional subscriber
    pub lookup_table_accounts: Vec<AddressLookupTableAccount>, 

}