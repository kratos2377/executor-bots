use std::{collections::{HashMap, HashSet}, sync::Arc, time::SystemTime};

use futures_util::lock::Mutex;
use num_bigint::BigInt;
use opentelemetry::metrics::{Histogram, Meter, ObservableGauge};
use spl_token::solana_program::pubkey::Pubkey;
use vortex_sdk::{usermap::UserMap, AccountProvider, AddressLookupTableAccount, VortexDexClient};

use crate::{global_config::LiquidatorConfig, metrics::RuntimeSpec, types::TwapExecutionProgress};


pub struct LiquidatorExecutor<T: AccountProvider, S> {
    pub name: String,
    pub dry_run: bool,
    pub default_interval_ms: u64,
    pub metrics_initialized: bool,
    pub metrics_port: Option<u64>,
    pub meter: Option<Meter>,

    // add prometheus exporter later

    pub throttled_users: HashMap<String, u64>,
    pub disable_auto_derisking: bool,
    pub liquidator_config: LiquidatorConfig,

    pub runtime_specs_gauge: Option<ObservableGauge<S>>,
    pub total_leverage: Option<ObservableGauge<S>>,
    pub total_collateral: Option<ObservableGauge<S>>,
    pub free_collateral: Option<ObservableGauge<S>>,
    pub initial_margin_requirement: Option<ObservableGauge<S>>,
    pub maintenance_margin_requirement: Option<ObservableGauge<S>>,
    pub unrealized_pnl: Option<ObservableGauge<S>>,
    pub unrealized_funding_pnl: Option<ObservableGauge<S>>,
    pub sdk_call_duration_histogram: Option<Histogram<S>>,
    pub user_map_user_account_keys_gauge: Option<ObservableGauge<S>>,

    pub vortex_client: VortexDexClient<T>,

    pub serum_lookup_table_address: Option<Pubkey>,
    pub drift_lookup_tables: Option<AddressLookupTableAccount>,
    pub drift_spot_lookup_tables: Option<AddressLookupTableAccount>,
    
    //perp_market_indices: Vec<u32>, // Assuming indices are of type u32
    pub spot_market_indices: Vec<u32>, // Assuming indices are of type u32
    pub active_sub_account_id: u32,
    pub all_subaccounts: HashSet<u32>,
    //perp_market_to_sub_account: HashMap<u32, u32>,
    pub spot_market_to_sub_account: HashMap<u32, u32>,
    pub interval_ids: Vec<tokio::time::Interval>, // Assuming you are using tokio for async timers
    pub user_map: UserMap,
    
    pub derisk_mutex: Arc<Mutex<u8>>, // Using Mutex for thread safety
    pub liquidate_mutex: Arc<Mutex<u8>>, // Using Mutex for thread safety
    pub runtime_specs: RuntimeSpec,
    pub twap_execution_progresses: Option<HashMap<String, TwapExecutionProgress>>, // Key-value pair
    pub excluded_accounts: HashSet<String>,

    pub priority_fee_subscriber: PriorityFeeSubscriber,

    pub max_position_takeover_pct_of_collateral_num: BigInt,
    pub max_position_takeover_pct_of_collateral_denom: BigInt,
    
    pub watchdog_timer_last_pat_time: SystemTime,


}