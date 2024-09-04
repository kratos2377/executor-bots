

use axum::async_trait;
use num_bigint::BigInt;
use num_traits::sign::Signed;
use vortex_contracts::state::position::PositionDirection;


pub struct MarketConstants {
    pub devent_usdc_mint: String,
    pub mainnet_usdc_mint: String,
}

impl MarketConstants {
    pub fn new() -> Self {
        MarketConstants {
            devent_usdc_mint: "8zGuJQqwhZafTah7Uc7Z4tXRnguqkn5KLFAP8oV6PHe2".to_string(),
            mainnet_usdc_mint: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v".to_string(),
        }
    }
}


pub enum OrderExecutionAlgoType {
    Market,
    Twap,
}

pub struct TwapExecutionConfig {
    pub current_position: BigInt,
    pub target_position: BigInt,
    pub overall_duration_sec: u8,
    pub start_time_sec: u8,
}


pub struct TwapExecutionProgress {
   pub amount_start: BigInt,
	pub current_position: BigInt,
	pub amount_target: BigInt,
	pub  overall_duration_sec: u8,
	pub start_time_sec: u8,
	pub last_update_sec: u8,
	pub last_exec_sec: u8,
	pub first_exec_done: bool,
}

impl TwapExecutionProgress {

    pub fn new(amount_start: BigInt , current_pos: BigInt, amount_target: BigInt , overall_duration: u8,
    start_time: u8, last_update: u8, last_exec: u8, first_exec: bool) -> Self {
        TwapExecutionProgress {
            amount_start: amount_start,
            current_position: current_pos,
            amount_target: amount_target,
            overall_duration_sec: overall_duration,
            start_time_sec: start_time,
            last_update_sec: last_update,
            last_exec_sec: last_exec,
            first_exec_done: first_exec,
        }
    }


    pub fn get_execution_slice(&self , now_sec: u8) -> BigInt {
        let order_size = self.amount_target.checked_sub(&self.amount_start).unwrap();
        let sec_elapsed = BigInt::from_signed_bytes_be(&[now_sec - self.last_exec_sec]);
    
        let slice = sec_elapsed.abs().checked_mul(&sec_elapsed).unwrap()
            .checked_div(&BigInt::from_signed_bytes_be(&[self.overall_duration_sec]))
            .unwrap().abs();

        if slice.gt(&order_size.abs()) {
            return  order_size.abs();
        }

        let remaining = self.get_amount_remaining();

        if remaining.lt(&slice) {
            return remaining;
        }

        slice
    }

    pub fn get_execution_direction(&self) -> PositionDirection {
        if self.amount_target.gt(&self.current_position) {
            return PositionDirection::Long;
        }

        return PositionDirection::Short
    }

    pub fn 	get_amount_remaining(&self) -> BigInt {
		return self.amount_target.checked_sub(&self.current_position).unwrap().abs();
	}

    pub fn update_progress(&mut self, current_pos: BigInt, now_sec: u8) {
        
        let curr_exec_size = self.amount_target.checked_sub(&self.amount_start).unwrap().abs();
        let new_exec_size = self.amount_target.checked_sub(&curr_exec_size).unwrap().abs();

        if new_exec_size.gt(&curr_exec_size) {
            self.amount_start = current_pos.clone();
            self.start_time_sec = now_sec;
        } 

        self.current_position = current_pos;
        self.last_update_sec = now_sec;
    }

    pub fn update_execution(&mut self, now_sec: u8) {
        self.last_exec_sec = now_sec;
        self.first_exec_done = true;
    }

    pub fn update_target(&mut self, new_target: BigInt, now_sec: u8) {
        self.amount_target = new_target;
        self.start_time_sec = now_sec;
        self.last_update_sec = now_sec;
    }

}

#[async_trait]
pub trait BotTrait {
    fn name(&self) -> &str; 
    fn dry_run(&self) -> bool; 
    fn default_interval_ms(&self) -> Option<u64>; 
    fn pyth_connection(&self) -> Option<&String>; 

    async fn init(&self);

 
    async fn reset(&self);

    async fn start_interval_loop(&self, interval_ms: Option<u64>);

   
    async fn health_check(&self) -> bool;
}

pub struct Executor {
    pub name: String,
    pub dry_run: bool,
    pub default_interval_ms: Option<u64>,
    pub pyth_connection: Option<String>,
}

#[async_trait]
impl BotTrait for Executor {
    fn name(&self) -> &str {
        &self.name
    }

    fn dry_run(&self) -> bool {
        self.dry_run
    }

    fn default_interval_ms(&self) -> Option<u64> {
        self.default_interval_ms
    }

    fn pyth_connection(&self) -> Option<&String> {
        self.pyth_connection.as_ref()
    }

    async fn init(&self) {
        // Implementation of initialization logic
    }

    async fn reset(&self) {
        // Implementation of reset logic
    }

    async fn start_interval_loop(&self, interval_ms: Option<u64>) {
        // Implementation of the polling loop
    }

    async fn health_check(&self) -> bool {
        // Implementation of health check logic
        true // Example return value
    }
}