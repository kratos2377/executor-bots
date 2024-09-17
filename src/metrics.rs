use std::collections::HashMap;

use opentelemetry::metrics::{Counter, Histogram, Meter, MeterProvider, ObservableGauge};
use vortex_sdk::AccountProvider;



pub struct GaugeValue<T> {
    pub latest_gauge_values: HashMap<String, u64>,
    pub gauge: ObservableGauge<T>
}


pub struct HistogramValue<T> {
    pub histogram: Histogram<T>
}

pub struct CounterValue<T> {
    pub counter: Counter<T>
}

pub struct RuntimeSpec {
    pub rpc_endpoint: String,
	pub vortex_env: String,
	pub commit: String,
	pub vortex_pid: String,
	pub wallet_authority: String,
}

pub struct VortexMeterProvider {

}


impl MeterProvider for VortexMeterProvider {
    fn versioned_meter(
        &self,
        name: impl Into<std::borrow::Cow<'static, str>>,
        version: Option<impl Into<std::borrow::Cow<'static, str>>>,
        schema_url: Option<impl Into<std::borrow::Cow<'static, str>>>,
        attributes: Option<Vec<opentelemetry::KeyValue>>,
    ) -> Meter {
        todo!()
    }
}


pub struct Metrics<T> {
    //prometheus exporter will be added later
    pub meter_provider: VortexMeterProvider,
    pub meters: HashMap<String , Meter>,
    pub gauges: Vec<GaugeValue<T>>,
    pub default_meter_name: String,
}