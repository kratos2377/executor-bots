use std::collections::HashMap;

use vortex_contracts::state::{dex_state::DexState, spot_market::SpotMarket};


pub struct StateChecks {
    pub new_spot_markets: bool,
    pub spot_market_status: bool,
}



pub struct VortexStateWatcher {
    pub last_state_account: Option<DexState>,
    pub last_spot_market_account: HashMap<u64, SpotMarket>,
    pub interval: Option<u64>,

    pub last_triggered: bool,
    pub last_triggered_states: StateChecks,
}