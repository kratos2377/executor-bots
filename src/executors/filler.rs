use std::{collections::HashMap, time::Duration};

use futures_util::lock::Mutex;
use lru::LruCache;
use num_bigint::BigInt;
use solana_client::nonblocking::tpu_connection::TpuConnection;
use vortex_contracts::state::user_map::{UserMap, UserStatsMap};
use vortex_sdk::{blockhash_subscriber::BlockhashSubscriber, dlob::dlob_node::{DLOBNode, Node, OrderNode, VAMMNode}, slot_subscriber::SlotSubscriber, AccountProvider, AddressLookupTableAccount, VortexDexClient};

use crate::{global_config::{FillerConfig, GlobalConfig}, metrics::{CounterValue, GaugeValue, HistogramValue, Metrics, RuntimeSpec}, vortex_price_feed_subscriber::VortexPriceFeedSubscriber};




pub enum MetricTypes {
    TRY_FILL_DURATION_HISTOGRAM,
    RUNTIME_SPECS,
    LAST_TRY_FILL_TIME,
    MUTEX_BUSY,
    SENT_TRANSACTIONS,
    LANDED_TRANSACTIONS,
    TX_SIM_ERROR_COUNT,
    PENDING_TX_SIGS_TO_CONFIRM,
    PENDING_TX_SIGS_LOOP_RATE_LIMITED,
    EVICTED_PENDING_TX_SIGS_TO_CONFIRM,
    ESTIMATED_TX_CU_HISTOGRTAM,
    SIMULATE_TX_DURATION_HISTOGRAM,
    EXPIRED_NODES_SET_SIZE,
    CLOCK_SUBSCRIBER_TS,
    WALL_CLOCK_TS

}

pub struct MakerNodeMap {
    pub node_map: HashMap<String , Vec<Node>>,
    pub order_node_map: HashMap<String , Vec<OrderNode>>,
    pub vamm_node_map: HashMap<String , Vec<VAMMNode>>,
}

pub enum TxType {
    FILL,
    TRIGGER,
    SETTLEPnL
}

impl TxType {
    pub fn as_str(&self) -> &str {
        match self {
            TxType::FILL => "fill",
            TxType::TRIGGER => "trigger",
            TxType::SETTLEPnL => "settlePnl",
        }
    }
}


pub struct SigsConfirmStruct {
    pub ts: u64,
    pub node_filled: Vec<NodeToFill>,
    pub fill_tx_id: u64,
    pub tx_type: TxType
}

pub struct FillerExecutor<'a, T: AccountProvider, S> {
    pub name: String,
    pub dry_run: bool,
    pub default_interval_ms: u64,
    pub slot_subscribe: SlotSubscriber,
    pub clock_subscriber: ClockSubscriber,
    pub bulk_account_loader: Option<BulkAccountLoader>,
    pub user_stats_map_subscription_config: UserSubscriptionConfig,

    pub vortex_client: VortexDexClient<T>,

    pub tx_confirmation_connection: ClientConnection,
    pub polling_interval_ms: u64,
    pub revert_on_failure: bool,
    pub simulate_tx_for_cu_estimate: bool,
    pub lookup_table_accounts: Vec<AddressLookupTableAccount>,
    pub bundle_sender: Option<BundleSender>,

    pub filler_config: FillerConfig,
    pub global_config: GlobalConfig,
    pub dlob_sub: Option<DTLSubscriber>,

    pub user_map: Option<UserMap<'a>>,
    pub user_stats_map: Option<UserStatsMap<'a>>,
    pub periodic_mutex: Mutex<u64>,
    pub watchdog_timer_mutex: Mutex<u64>,
    pub wathdog_timer_last_pat_time: Duration,

    pub interval_ids: Vec<u64>,
    pub throttled_nodes: HashMap<String , u64>,
    pub filling_nodes: HashMap<String , u64>,
    pub triggering_nodes: HashMap<String , u64>,

    pub user_burst_cu_limit: bool,
    pub fill_tx_since_burst_cu: u64,
    pub fill_tx_id: u64,
    pub last_settle_pnl: Duration,

    pub priority_fee_sub: PriorityFeeSubscriber,
    pub blockhash_subscriber: BlockhashSubscriber,

    pub pending_tx_sigs_to_confirm: LruCache<String , SigsConfirmStruct>,
    pub expired_nodes_set: LruCache<String , bool>,
    pub confirm_loop_running: bool,
    pub confirm_loop_rate_limit_ts: Duration,


    pub metrics_initialized: bool,
    pub metrics_port: Option<u64>,
    pub metrics: Option<Metrics<S>>,
    pub boot_time_ms: Option<u64>,

    pub runtime_spec: RuntimeSpec,
    pub runtime_specs_gauge: Option<GaugeValue<S>>,
    pub try_fill_duration_histogram: Option<HistogramValue<S>>,
    pub est_tx_cu_histogram: Option<HistogramValue<S>>,
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
    pub clock_subscriber_ts: Option<GaugeValue<S>>,
    pub wall_clock_ts: Option<GaugeValue<S>>,

    pub has_enought_sol_to_fill: bool,
    pub rebalance_filler: bool,
    pub min_gas_balance_to_fill: u64,
    pub rebalance_settle_pnl_threshold: BigInt,

    pub price_feed_subscriber: VortexPriceFeedSubscriber


}