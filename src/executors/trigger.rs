use std::collections::HashMap;

use futures_util::lock::Mutex;
use opentelemetry::metrics::{Counter, Histogram, Meter, ObservableGauge};
use vortex_sdk::{common::priority_fee::PriorityFeeCalculator, dtl_subscriber::dtl_subscriber::DTLSubscriber, slot_subscriber::SlotSubscriber, usermap::UserMap, AccountProvider, VortexDexClient};

use crate::metrics::RuntimeSpec;




pub struct TriggerExecutor<T: AccountProvider, S> {
    pub name: String,
    pub dry_run: bool,
    pub default_interval_ms: u64, // Assuming milliseconds is represented as u64

    pub vortex_client: VortexDexClient<T>,
    pub slot_subscriber: SlotSubscriber,
    pub dlob_subscriber: Option<DTLSubscriber<T>>, // Optional subscriber
    pub triggering_nodes: HashMap<String, u32>, // Using HashMap for key-value pairs
    pub periodic_task_mutex: Mutex<()>, // Mutex for synchronization
    pub interval_ids: Vec<u128>, // Assuming NodeJS_Timer is defined elsewhere
    pub user_map: UserMap,

    pub priority_fee_calculator: PriorityFeeCalculator,

    // Metrics
    pub metrics_initialized: bool,
    pub metrics_port: Option<u16>, // Optional port number
 //   pub exporter: Option<PrometheusExporter>, // Optional exporter
    pub meter: Option<Meter>, // Optional meter
    pub boot_time_ms: u128, // Boot time in milliseconds since epoch
    pub runtime_specs_gauge: Option<ObservableGauge<S>>, // Optional gauge
    pub runtime_spec: RuntimeSpec,
    pub mutex_busy_counter: Option<Counter<S>>, // Optional counter
    pub error_counter: Option<Counter<S>>, // Optional counter
    pub try_trigger_duration_histogram: Option<Histogram<S>>, // Optional histogram

    pub watchdog_timer_mutex: Mutex<()>, // Mutex for synchronization
    pub watchdog_timer_last_pat_time: u128
}