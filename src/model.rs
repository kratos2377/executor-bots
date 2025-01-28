use serde::{Serialize , Deserialize};

#[derive(Clone , Serialize , Deserialize)]
pub struct GameBetSettleKafkaPayload {
    pub game_id: String,
    pub session_id: String,
    pub winner_id: String,
    pub user_id: String,
    pub user_betting_on: String,
    pub record_id: String,
    pub user_wallet_key: String,
}



#[derive(Clone , Serialize , Deserialize)]
pub struct EventRecord {
    pub payload: String,
    pub event_type: String,
}


//Event Types
pub const EXECUTOR_INDEX_ADD: &str = "executor_index_add";
pub const CONSUMER_GROUP_BET_EVENT_ADD: &str = "consumer_group_bet_event_add";