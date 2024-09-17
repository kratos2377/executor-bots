use std::collections::{HashMap, HashSet};

use num_bigint::BigInt;

use crate::types::OrderExecutionAlgoType;



pub struct BaseExecutorConfig {
    pub bot_id: String,
    pub dry_run: bool,
    pub metrics_port: Option<u64>,
    pub run_once: Option<bool>,
}

pub struct UserPnLSettlerConfig {
    pub base_config: BaseExecutorConfig,
    pub market_indices: Option<Vec<u128>>,
    pub settle_pnl_usdc_threshold: Option<u128>,
    pub max_users_to_consider: Option<u128>
}

pub struct FillerMultiThreadedConfig {
    pub base_config: BaseExecutorConfig,
    pub market_type: String,
    pub market_indices: Vec<Vec<u64>>,
    pub simulate_tx_for_cu_estimate: Option<bool>,
    pub revert_on_failure: Option<bool>,
    pub sub_account: Option<u128>,
    pub rebalance_filler: Option<bool>,
    pub rebalance_settled_pnl_threshold: Option<u64>,
    pub min_gas_balance_to_fill: Option<u64>,
}


pub struct FillerConfig {
    pub base_config: BaseExecutorConfig,
    pub filler_polling_interval: Option<u64>,
    pub simulate_tx_for_cu_estimate: Option<bool>,
    pub rebalance_filler: Option<bool>,
    pub rebalace_settled_pnl_threshold: Option<u64>,
    pub min_gas_balance_to_fill: Option<u64>,
}

pub struct SubAccountConfig {
    pub sub_configs: HashMap<u64 , Vec<u64>>
}

pub struct LiquidatorConfig {
    pub base_config: BaseExecutorConfig,
    pub disableAutoDerisking: bool,
	pub use_jupiter: bool,

	pub spot_sub_account_config: Option<SubAccountConfig>,

	// deprecated: use {@link LiquidatorConfig.maxSlippageBps} (misnamed)
	pub max_slippage_pct: Option<u64>,
	pub max_slippage_bps: Option<u64>,

	pub derisk_auction_duration_slots: Option<u64>,
	pub derisk_algo: Option<OrderExecutionAlgoType>,
	pub derisk_algo_spot: Option<OrderExecutionAlgoType>,
	pub  twap_duration_sec: Option<u64>,
	pub min_deposit_to_liq: Option<HashMap<u64, u64>>,
	pub excluded_accounts: Option<HashSet<String>>,
	pub max_position_takeover_pct_of_collateral: Option<u64>,
	pub notify_on_liquidation: Option<bool>,


	pub spot_dust_value_threshold: Option<u64>,
	/// Placeholder, liquidator will set this to the raw BigInt of {@link LiquidatorConfig.spotDustValueThreshold}
	pub spot_dust_value_threshold_bn: Option<BigInt>,
}


pub struct ExecutorConfigMap {
    pub filler_multithreaded: Option<FillerMultiThreadedConfig>,
    pub spot_filler_multithreaded: Option<FillerMultiThreadedConfig>,
    pub filler: Option<FillerConfig>,
    pub filler_lite: Option<FillerConfig>,
    pub spot_filler: Option<FillerConfig>,
    pub trigger: Option<BaseExecutorConfig>,
    pub liquidator: Option<LiquidatorConfig>,
    pub floating_maker: Option<BaseExecutorConfig>,
    pub funding_rate_updater: Option<BaseExecutorConfig>,
    pub user_pnl_settler: Option<UserPnLSettlerConfig>,
    pub user_lp_settler: Option<BaseExecutorConfig>,
    pub user_idle_flipper: Option<BaseExecutorConfig>,
    pub mark_twap_crank: Option<BaseExecutorConfig>,
    pub uncross_arb: Option<BaseExecutorConfig>,
    
}


pub struct GlobalConfig {
    pub vortex_env: VortexEnvironmentConfig,
    pub endpoint: String,
    pub ws_endpoint: Option<String>,
    pub hermes_endpoint: Option<String>,
    pub num_non_active_oracles_to_pus: Option<u128>,

    pub helius_endpoint: Option<String>,

    pub additional_send_tx_endpoints: Option<Vec<String>>,
    pub tx_confirmation_endpoint: Option<String>,

    pub metrics_port: Option<u128>,
    pub disable_metics: Option<bool>,

    pub priority_fee_method: Option<String>,


    pub max_priority_fee_micro_lamports: Option<u64>, // Converted from u64
    pub resub_timeout_ms: Option<u64>,                // Converted from u64
    pub priority_fee_multiplier: Option<u64>,         // Converted from u64
    pub keeper_private_key: Option<String>,            // Converted from String
    pub init_user: Option<bool>,                       // Converted from bool
    pub test_liveness: Option<bool>,                   // Converted from bool
    pub cancel_open_orders: Option<bool>,             // Converted from bool
    pub close_open_positions: Option<bool>,           // Converted from bool
    pub force_deposit: Option<Option<u64>>,           // Converted from u64 | null
    pub websocket: Option<bool>,                       // Converted from bool
    pub event_subscriber: bool,                        // Always false in TypeScript
    pub run_once: Option<bool>,                        // Converted from bool
    pub debug: Option<bool>,                           // Converted from bool
    pub subaccounts: Option<Vec<u64>>,                // Converted from Vec<u64>

    pub event_subscriber_polling_interval: u64,      // Converted from u64
    pub bulk_account_loader_polling_interval: u64,   // Converted from u64


    pub tx_retry_timeout_ms: Option<u64>,             // Converted from u64
    pub tx_sender_type: Option<TxSenderType>,         // Enum for 'fast', 'retry', 'while-valid'
    pub tx_skip_preflight: Option<bool>,               // Converted from bool
    pub tx_max_retries: Option<u64>,                   // Converted from u64
    pub track_tx_land_rate: Option<bool>,              // Converted from bool

    pub rebalance_filler: Option<bool>,                // Converted from bool

}

pub enum TxSenderType {
    Fast,
    Retry,
    WhileValid
}


pub struct ExecutorConfig {
    pub global_config: GlobalConfig,
    pub enabled_bots: Vec<ExecutorConfigMap>,
    pub bot_configs: Option<ExecutorConfigMap>,
}